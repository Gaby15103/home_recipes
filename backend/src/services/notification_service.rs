use crate::app::state::AppState;
use crate::dto::notification_dto::{
    CreateNotificationTemplateInput, NotificationListResponse, NotificationResponse,
    NotificationTemplateResponse, NotificationTrigger,
};
use crate::errors::Error;
use crate::repositories::{
    notification_repository, notification_template_repository, user_repository,
};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use uuid::Uuid;

/// Retrieves notifications for a specific user, including unread count.
pub async fn get_for_user(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<NotificationListResponse, Error> {
    let notifications = notification_repository::find_by_user(db, user_id).await?;
    let unread_count = notification_repository::count_unread(db, user_id).await?;

    let mut response_items = Vec::new();

    for model in notifications {
        let mut actor_name = None;

        if let Some(a_id) = model.actor_id {
            if let Ok(actor) = user_repository::find_by_id(db, a_id).await {
                actor_name = Some(actor.username);
            }
        }

        response_items.push(NotificationResponse::new(model, actor_name));
    }

    Ok(NotificationListResponse {
        items: response_items,
        unread_count,
    })
}

/// Marks a single notification as read.
pub async fn mark_as_read(
    db: &DatabaseConnection,
    notification_id: Uuid,
    user_id: Uuid,
) -> Result<(), Error> {
    notification_repository::mark_as_read(db, notification_id, user_id).await?;
    Ok(())
}

/// Marks all notifications as read for a user.
pub async fn mark_all_read(db: &DatabaseConnection, user_id: Uuid) -> Result<(), Error> {
    notification_repository::mark_all_read(db, user_id).await?;
    Ok(())
}

/// Creates a new notification template (Admin/Mod only).
pub async fn create_template(
    db: &DatabaseConnection,
    input: CreateNotificationTemplateInput,
) -> Result<NotificationTemplateResponse, Error> {
    let template = notification_template_repository::create(db, input).await?;
    Ok(NotificationTemplateResponse::from(template))
}

/// The main entry point to trigger a new notification.
/// This handles logic for: Preferences -> Translation Fallback -> Storage -> WS Broadcast.
pub async fn trigger(state: &AppState, trigger: NotificationTrigger) -> Result<(), Error> {
    let db = &state.db;

    let user = user_repository::find_by_id(db, trigger.recipient_id).await?;

    let lang = user.preferences["language"]
        .as_str()
        .unwrap_or("en")
        .to_string();

    let is_enabled = user.preferences["notifications"][&trigger.category]
        .as_bool()
        .unwrap_or(true);

    if !is_enabled {
        return Ok(());
    }

    let (title_tpl, msg_tpl) = get_or_translate_template(state, &trigger.category, &lang).await?;

    let final_title = inject_variables(title_tpl, &trigger.variables);
    let final_message = inject_variables(msg_tpl, &trigger.variables);

    let saved_notif = notification_repository::create(
        db,
        trigger.recipient_id,
        trigger.actor_id,
        trigger.category,
        final_title,
        final_message,
        trigger.target_id,
    )
        .await?;

    let mut actor_name = None;
    if let Some(a_id) = trigger.actor_id {
        actor_name = trigger.variables.get("actor").cloned()
            .or_else(|| {
                None
            });

        if actor_name.is_none() {
            if let Ok(actor) = user_repository::find_by_id(db, a_id).await {
                actor_name = Some(actor.username);
            }
        }
    }

    let response = NotificationResponse {
        id: saved_notif.id,
        user_id: saved_notif.user_id,
        actor_id: saved_notif.actor_id,
        actor_name,
        category: saved_notif.category,
        title: saved_notif.title,
        message: saved_notif.message,
        target_id: saved_notif.target_id,
        is_read: saved_notif.is_read,
        created_at: saved_notif.created_at.into(),
    };

    let ws_payload = serde_json::to_string(&response).unwrap_or_default();

    // DEBUG START
    println!("--- NOTIFICATION DEBUG ---");
    println!("Recipient ID: {}", trigger.recipient_id);

    let hub_clients = state.notification_hub.clients.read().await;
    println!("Hub contains {} active user IDs", hub_clients.keys().count());

    if hub_clients.contains_key(&trigger.recipient_id) {
        println!("MATCH FOUND: Sending live notification to hub.");
    } else {
        println!("ERROR: Recipient ID {} is NOT connected to the hub.", trigger.recipient_id);
        println!("Connected IDs are: {:?}", hub_clients.keys().collect::<Vec<_>>());
    }
    // DEBUG END

    state.notification_hub.broadcast_to_user(trigger.recipient_id, ws_payload).await;

    Ok(())
}

/// Internal: Handles the lookup or automated translation of templates.
async fn get_or_translate_template(
    state: &AppState,
    category: &str,
    lang: &str,
) -> Result<(String, String), Error> {
    let db = &state.db;

    // Try finding existing translation in the repository
    if let Some(tpl) =
        notification_template_repository::find_by_category_lang(db, category, lang).await?
    {
        return Ok((tpl.title_template, tpl.message_template));
    }

    // Fallback: Get English base
    let en_tpl = notification_template_repository::find_by_category_lang(db, category, "en")
        .await?
        .ok_or_else(|| {
            Error::NotFound(format!("Base English template for {} not found", category).into())
        })?;

    // Call LibreTranslate
    let translated_title = call_libretranslate(state, &en_tpl.title_template, lang).await?;
    let translated_msg = call_libretranslate(state, &en_tpl.message_template, lang).await?;

    // Cache the new translation so we don't hit the API again
    notification_template_repository::create(
        db,
        CreateNotificationTemplateInput {
            category: category.to_string(),
            language_code: lang.to_string(),
            title_template: translated_title.clone(),
            message_template: translated_msg.clone(),
        },
    )
    .await?;

    Ok((translated_title, translated_msg))
}

/// Internal: Calls your local LibreTranslate instance.
async fn call_libretranslate(
    state: &AppState,
    text: &str,
    target_lang: &str,
) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let url = format!("{}", state.config.translator_url);

    let res = client
        .post(&url)
        .json(&serde_json::json!({
            "q": text,
            "source": "en",
            "target": target_lang,
            "format": "text"
        }))
        .send()
        .await
        .map_err(|e| {
            Error::InternalServerError(
                serde_json::json!({
                    "msg": "LibreTranslate unreachable",
                    "url": url,
                    "error": e.to_string()
                }),
            )
        })?;

    let status = res.status();
    let raw_body = res.text().await.map_err(|e| {
        Error::InternalServerError(
            serde_json::json!({"msg": "Failed to read body", "error": e.to_string()}),
        )
    })?;

    let json_body: serde_json::Value = serde_json::from_str(&raw_body).map_err(|e| {
        Error::InternalServerError(
            serde_json::json!({
                "msg": "Failed to parse LibreTranslate response",
                "status": status.as_u16(),
                "raw_response": raw_body, // This is key for debugging!
                "error": e.to_string()
            }),
        )
    })?;

    if !status.is_success() {
        return Err(Error::InternalServerError(
            serde_json::json!({
                "msg": "LibreTranslate returned error status",
                "status": status.as_u16(),
                "response": json_body
            })
        ));
    }

    Ok(json_body["translatedText"]
        .as_str()
        .unwrap_or(text)
        .to_string())
}
/// Internal: Replaces {key} in strings with values from the trigger HashMap.
fn inject_variables(mut template: String, variables: &HashMap<String, String>) -> String {
    for (key, value) in variables {
        template = template.replace(&format!("{{{}}}", key), value);
    }
    template
}

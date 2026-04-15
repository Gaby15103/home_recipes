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
        let mut variables = HashMap::new();

        let mut actor_name = None;

        match model.category.as_str() {
            "recipe_comment" | "comment_reply" | "recipe_favorite" => {
                if let Some(a_id) = model.actor_id {
                    if let Ok(actor) = user_repository::find_by_id(db, a_id).await {
                        let name = actor.username;
                        actor_name = Some(name.clone());
                        variables.insert("actor".to_string(), name);
                    }
                }

                if let Some(r_id) = model.target_id {
                    if let Ok(translation) = crate::repositories::recipe_translation_repository::find_translation(
                        db, r_id, "en", "en"
                    ).await {
                        variables.insert("recipe_title".to_string(), translation.title);
                    }
                }
            },
            _ => {}
        }
        response_items.push(NotificationResponse::new(model, actor_name, variables));
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
    let lang = user.preferences["language"].as_str().unwrap_or("en").to_string();

    if !user.preferences["notifications"][&trigger.category].as_bool().unwrap_or(true) {
        return Ok(());
    }

    // 1. Get English base (un-injected)
    let (title_tpl, msg_tpl) = get_or_translate_template(state, &trigger.category, "en").await?;

    // 2. Translate the templates while they still have {actor} and {recipe_title}
    let (translated_title, translated_msg) = if lang == "en" {
        (title_tpl, msg_tpl)
    } else {
        // 1. Armor with symbols that translators usually ignore
        let p_title = title_tpl.replace("{actor}", "[#A#]").replace("{recipe_title}", "[#R#]");
        let p_msg = msg_tpl.replace("{actor}", "[#A#]").replace("{recipe_title}", "[#R#]");

        let t_title = call_libretranslate(state, &p_title, &lang).await?;
        let t_message = call_libretranslate(state, &p_msg, &lang).await?;

        // 2. Flexible Restoration
        // We handle potential spaces added by the translator (e.g. "[# A #]")
        let restore = |s: String| {
            s.replace("[#A#]", "{actor}")
                .replace("[# A #]", "{actor}")
                .replace("[#R#]", "{recipe_title}")
                .replace("[# R #]", "{recipe_title}")
        };

        (restore(t_title), restore(t_message))
    };

    // 3. Inject ONLY the comment_preview (so it stays raw and untranslated)
    let raw_comment = trigger.variables.get("comment_preview").cloned().unwrap_or_default();
    let final_message = translated_msg
        .replace("{comment_preview}", &raw_comment)
        .replace("{comment preview}", &raw_comment); // Handle LibreTranslate space quirk

    // 4. Save to DB (The message still contains {actor} and {recipe_title})
    let saved_notif = notification_repository::create(
        db,
        trigger.recipient_id,
        trigger.actor_id,
        trigger.category,
        translated_title, // Keep placeholders here too
        final_message,
        trigger.target_id,
    ).await?;

    // 5. Build Response with variables for the frontend
    let response = NotificationResponse {
        id: saved_notif.id,
        user_id: saved_notif.user_id,
        actor_id: saved_notif.actor_id,
        actor_name: trigger.variables.get("actor").cloned(),
        category: saved_notif.category,
        title: saved_notif.title,
        message: saved_notif.message, // Contains "{actor} replied on {recipe_title}: my comment"
        target_id: saved_notif.target_id,
        is_read: saved_notif.is_read,
        created_at: saved_notif.created_at.into(),
        variables: trigger.variables.clone(), // Send the map to Vue
    };

    let ws_payload = serde_json::to_string(&response).unwrap_or_default();
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

    if let Some(tpl) = notification_template_repository::find_by_category_lang(db, category, lang).await? {
        return Ok((tpl.title_template, tpl.message_template));
    }

    let en_tpl = notification_template_repository::find_by_category_lang(db, category, "en")
        .await?
        .ok_or_else(|| Error::NotFound(format!("Base English template for {} not found", category).into()))?;

    // SWAP: Protect during the initial translation/caching process
    let p_title = en_tpl.title_template.replace("{actor}", "[#A#]").replace("{recipe_title}", "[#R#]");
    let p_msg = en_tpl.message_template.replace("{actor}", "[#A#]").replace("{recipe_title}", "[#R#]");

    let t_title_raw = call_libretranslate(state, &p_title, lang).await?;
    let t_msg_raw = call_libretranslate(state, &p_msg, lang).await?;

    // Restore
    let translated_title = t_title_raw.replace("[#A#]", "{actor}").replace("[# A #]", "{actor}")
        .replace("[#R#]", "{recipe_title}").replace("[# R #]", "{recipe_title}");
    let translated_msg = t_msg_raw.replace("[#A#]", "{actor}").replace("[# A #]", "{actor}")
        .replace("[#R#]", "{recipe_title}").replace("[# R #]", "{recipe_title}");

    notification_template_repository::create(
        db,
        CreateNotificationTemplateInput {
            category: category.to_string(),
            language_code: lang.to_string(),
            title_template: translated_title.clone(),
            message_template: translated_msg.clone(),
        },
    ).await?;

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

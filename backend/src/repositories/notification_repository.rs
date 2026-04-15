use crate::dto::notification_dto::CreateNotificationTemplateInput;
use crate::errors::Error;
use entity::{notification_templates, notifications};
use sea_orm::PaginatorTrait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use serde_json::json;
use uuid::Uuid;

/// Find all notifications for a specific user
pub async fn find_by_user(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<notifications::Model>, Error> {
    notifications::Entity::find()
        .filter(notifications::Column::UserId.eq(user_id))
        .order_by_desc(notifications::Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| {
            Error::InternalServerError(json!({
                "message": "Failed to fetch user notifications",
                "operation": "find_by_user",
                "entity": "notifications",
                "user_id": user_id.to_string(),
                "error": e.to_string()
            }))
        })
}

/// Get the count of unread notifications for a user
pub async fn count_unread(db: &DatabaseConnection, user_id: Uuid) -> Result<i64, Error> {
    let count = notifications::Entity::find()
        .filter(notifications::Column::UserId.eq(user_id))
        .filter(notifications::Column::IsRead.eq(false))
        .count(db)
        .await
        .map_err(|e| {
            Error::InternalServerError(json!({
                "message": "Failed to count unread notifications",
                "user_id": user_id.to_string(),
                "error": e.to_string()
            }))
        })?;

    Ok(count as i64)
}

/// Create a new notification record
pub async fn create(
    db: &DatabaseConnection,
    user_id: Uuid,
    actor_id: Option<Uuid>,
    category: String,
    title: String,
    message: String,
    target_id: Option<Uuid>,
) -> Result<notifications::Model, Error> {
    let category_for_error = category.clone();
    notifications::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        actor_id: Set(actor_id),
        category: Set(category),
        title: Set(title),
        message: Set(message),
        target_id: Set(target_id),
        is_read: Set(false),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(|e| {
        Error::InternalServerError(json!({
            "message": "Failed to create notification",
            "user_id": user_id.to_string(),
            "category": category_for_error,
            "error": e.to_string()
        }))
    })
}

/// Mark a specific notification as read (ensuring it belongs to the user)
pub async fn mark_as_read(db: &DatabaseConnection, id: Uuid, user_id: Uuid) -> Result<(), Error> {
    notifications::Entity::update_many()
        .col_expr(
            notifications::Column::IsRead,
            sea_orm::prelude::Expr::value(true),
        )
        .filter(notifications::Column::Id.eq(id))
        .filter(notifications::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(|e| {
            Error::InternalServerError(json!({
                "message": "Failed to mark notification as read",
                "id": id.to_string(),
                "error": e.to_string()
            }))
        })?;

    Ok(())
}

/// Mark all unread notifications as read for a specific user
pub async fn mark_all_read(db: &DatabaseConnection, user_id: Uuid) -> Result<(), Error> {
    notifications::Entity::update_many()
        .col_expr(
            notifications::Column::IsRead,
            sea_orm::prelude::Expr::value(true),
        )
        .filter(notifications::Column::UserId.eq(user_id))
        .filter(notifications::Column::IsRead.eq(false))
        .exec(db)
        .await
        .map_err(|e| {
            Error::InternalServerError(json!({
                "message": "Failed to mark all notifications as read",
                "user_id": user_id.to_string(),
                "error": e.to_string()
            }))
        })?;

    Ok(())
}

// --- Template Repository Logic ---

/// Find a notification template by category and language
pub async fn find_template_by_category_lang(
    db: &DatabaseConnection,
    category: &str,
    lang: &str,
) -> Result<Option<notification_templates::Model>, Error> {
    notification_templates::Entity::find()
        .filter(notification_templates::Column::Category.eq(category))
        .filter(notification_templates::Column::LanguageCode.eq(lang))
        .one(db)
        .await
        .map_err(|e| {
            Error::InternalServerError(json!({
                "message": "Failed to fetch template",
                "category": category,
                "lang": lang,
                "error": e.to_string()
            }))
        })
}

/// Save a new notification template (used for manual creation or caching translations)
pub async fn create_template(
    db: &DatabaseConnection,
    input: CreateNotificationTemplateInput,
) -> Result<notification_templates::Model, Error> {
    notification_templates::ActiveModel {
        id: Set(Uuid::new_v4()),
        category: Set(input.category),
        language_code: Set(input.language_code),
        title_template: Set(input.title_template),
        message_template: Set(input.message_template),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(|e| {
        Error::InternalServerError(json!({
            "message": "Failed to create notification template",
            "error": e.to_string()
        }))
    })
}

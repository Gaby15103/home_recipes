use crate::dto::notification_dto::CreateNotificationTemplateInput;
use crate::errors::Error;
use entity::notification_templates;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

/// Retrieves a template by its category and language code.
/// Used for fetching cached translations or the base English template.
pub async fn find_by_category_lang(
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
                "message": "Failed to fetch notification template",
                "operation": "find_by_category_lang",
                "entity": "notification_templates",
                "category": category,
                "language": lang,
                "error": e.to_string(),
                "stage": "database_query"
            }))
        })
}

/// Creates a new notification template entry.
/// This is used both for admin manual setup and for caching automated translations.
pub async fn create(
    db: &DatabaseConnection,
    input: CreateNotificationTemplateInput,
) -> Result<notification_templates::Model, Error> {
    let active_model = notification_templates::ActiveModel {
        id: Set(Uuid::new_v4()),
        category: Set(input.category.clone()),
        language_code: Set(input.language_code.clone()),
        title_template: Set(input.title_template),
        message_template: Set(input.message_template),
        ..Default::default()
    };

    active_model.insert(db).await.map_err(|e| {
        Error::InternalServerError(json!({
            "message": "Failed to insert notification template",
            "operation": "create",
            "entity": "notification_templates",
            "category": input.category,
            "language": input.language_code,
            "error": e.to_string(),
            "stage": "insert"
        }))
    })
}

/// Lists all available templates (useful for Admin dashboards).
pub async fn find_all(
    db: &DatabaseConnection,
) -> Result<Vec<notification_templates::Model>, Error> {
    notification_templates::Entity::find()
        .all(db)
        .await
        .map_err(|e| {
            Error::InternalServerError(json!({
                "message": "Failed to fetch all notification templates",
                "operation": "find_all",
                "entity": "notification_templates",
                "error": e.to_string(),
                "stage": "database_query"
            }))
        })
}

/// Deletes a template by ID.
pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<bool, Error> {
    let result = notification_templates::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| {
            Error::InternalServerError(json!({
                "message": "Failed to delete notification template",
                "operation": "delete",
                "entity": "notification_templates",
                "id": id.to_string(),
                "error": e.to_string(),
                "stage": "delete_execution"
            }))
        })?;

    Ok(result.rows_affected > 0)
}

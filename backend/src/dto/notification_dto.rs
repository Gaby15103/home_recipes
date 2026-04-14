use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use entity::{notifications, notification_templates};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub actor_id: Option<Uuid>,
    pub actor_name: Option<String>,
    pub category: String,
    pub title: String,
    pub message: String,
    pub target_id: Option<Uuid>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

impl NotificationResponse {
    pub fn new(model: notifications::Model, actor_name: Option<String>) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            actor_id: model.actor_id,
            actor_name, // Injected from user lookup
            category: model.category,
            title: model.title,
            message: model.message,
            target_id: model.target_id,
            is_read: model.is_read,
            created_at: model.created_at.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NotificationTemplateResponse {
    pub id: Uuid,
    pub category: String,
    pub language_code: String,
    pub title_template: String,
    pub message_template: String,
}

impl From<notification_templates::Model> for NotificationTemplateResponse {
    fn from(model: notification_templates::Model) -> Self {
        Self {
            id: model.id,
            category: model.category,
            language_code: model.language_code,
            title_template: model.title_template,
            message_template: model.message_template,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationActorDto {
    pub id: Uuid,
    pub username: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateNotificationTemplateInput {
    pub category: String,
    pub language_code: String,
    pub title_template: String,
    pub message_template: String,
}

#[derive(Debug, Clone)]
pub struct NotificationTrigger {
    pub recipient_id: Uuid,
    pub actor_id: Option<Uuid>,
    pub category: String,
    pub target_id: Option<Uuid>,
    pub variables: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct LibreTranslateRequest<'a> {
    pub q: &'a str,
    pub source: &'a str,
    pub target: &'a str,
    pub format: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct LibreTranslateResponse {
    #[serde(rename = "translatedText")]
    pub translated_text: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationListResponse {
    pub items: Vec<NotificationResponse>,
    pub unread_count: i64,
}
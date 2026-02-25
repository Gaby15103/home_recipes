use chrono::{DateTime, Utc};
use entity::sessions;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct SessionDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: Uuid,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub is_revoked: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_active_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionResponseDto {
    pub id: Uuid,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub is_revoked: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_active_at: Option<DateTime<Utc>>,
    pub is_current: bool,
}

impl SessionResponseDto {
    pub fn from_model(model: sessions::Model, current_session_id: Uuid) -> Self {
        Self {
            id: model.id,
            user_agent: model.user_agent,
            ip_address: model.ip_address,
            is_revoked: model.is_revoked,
            expires_at: model.expires_at.into(),
            created_at: model.created_at.map(|dt| dt.into()).unwrap_or_else(Utc::now),
            last_active_at: model.last_active_at.map(|dt| dt.into()),
            is_current: model.id == current_session_id,
        }
    }
}
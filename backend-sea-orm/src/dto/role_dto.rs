use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use entity::{roles, users};
use crate::dto::preferences_dto::UserPreferences;
use crate::dto::user_dto::UserResponseDto;

#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct RoleResponseDto {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
impl From<(roles::Model)> for RoleResponseDto {
    fn from((roles): (entity::roles::Model)) -> Self {
        Self {
            id: roles.id,
            name: roles.name,
            description: roles.description,
        }
    }
}
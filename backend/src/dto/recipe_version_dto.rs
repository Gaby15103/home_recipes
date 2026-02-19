use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use entity::recipe_versions;
use crate::dto::recipe_dto::RecipeEditorDto;
use crate::dto::user_dto::UserResponseDto;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RecipeVersionDto {
    pub id: Uuid,
    pub data: RecipeEditorDto,
    pub recipe_id: Uuid,
    pub edited_by: UserResponseDto,
    pub created_at:DateTime<Utc>,
}
impl RecipeVersionDto {
    pub fn from_model(version: recipe_versions::Model, user: UserResponseDto) -> Self {
        let recipe_data: RecipeEditorDto = serde_json::from_value(version.data)
            .unwrap_or_default();

        Self {
            id: version.id,
            data: recipe_data,
            recipe_id: version.recipe_id,
            edited_by: user,
            created_at: version.created_at.with_timezone(&Utc),
        }
    }
}
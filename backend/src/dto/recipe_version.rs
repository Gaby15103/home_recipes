use chrono::DateTime;
use uuid::Uuid;
use crate::dto::RecipeResponse;
use crate::models::Recipe;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeVersionResponse {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub recipe: RecipeResponse,
    pub created_at: DateTime<chrono::Utc>,
    pub edited_by: Uuid,
}
pub struct GetRecipeVersions {
    pub recipe_id: Uuid,
}
pub struct GetRecipeVersion {
    pub id: Uuid,
}
pub struct RestoreRecipeVersion {
    pub recipe_id: Uuid,
    pub version_id: Uuid,
    pub user_id: Uuid,
    pub language_code: String,
}
impl RecipeVersionResponse {
    pub fn from_tuple(
        (version_id, recipe_id, recipe, created_at, edited_by):
        (Uuid, Uuid, RecipeResponse, chrono::NaiveDateTime, Uuid)
    ) -> Self {
        Self {
            id: version_id,
            recipe_id,
            recipe,
            created_at: chrono::DateTime::<chrono::Utc>::from_utc(created_at, chrono::Utc),
            edited_by,
        }
    }
}


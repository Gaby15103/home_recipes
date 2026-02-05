use chrono::DateTime;
use uuid::Uuid;
use crate::schema::step_translations::language_code;

pub struct FavoriteResponse {
    pub recipe_id: Uuid,
    pub created_at: DateTime<chrono::Utc>,
}
pub struct ToggleFavorite {
    pub user_id: Uuid,
    pub recipe_id: Uuid,
}

pub struct UnfavoriteRecipe {
    pub recipe_id: Uuid,
    pub user_id: Uuid,
}

pub struct GetFavoriteRecipes {
    pub user_id: Uuid,
    pub language_code: String,
}

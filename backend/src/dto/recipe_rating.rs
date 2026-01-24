use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeRatingResponse {
    pub average: f32,
    pub count: i64,
    pub user_rating: Option<i32>,
}
pub struct SetRecipeRating {
    pub user_id: Uuid,
    pub recipe_id: Uuid,
    pub rating: i32,
}
pub struct GetRecipeRating {
    pub recipe_id: Uuid,
    pub user_id: Option<Uuid>,
}

pub struct UnsetRecipeRating {
    pub recipe_id: Uuid,
    pub user_id: Uuid,
}
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct RecipeRatingDto {
    pub average: f32,
    pub count: i64,
    pub user_rating: Option<i32>,
}
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, ToSchema, Clone)]
pub struct DashboardStats {
    pub total_recipes: i32,
    pub public_recipes: i32,
    pub private_recipes: i32,
    pub total_views: i32,
}
// crate::dto::studio_dto
#[derive(Serialize)]
pub struct RecipeTelemetry {
    pub recipe_id: Uuid,
    pub step_count: i32,
    pub ingredient_count: i32,
    pub complexity_score: f32,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Serialize)]
pub struct RecipeAnalytics {
    pub total_views: i64,
    pub views_7d: Vec<i32>,
    pub avg_session_duration: String,
    pub print_count: i32,
    pub share_count: i32,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub health_score: f32,
}
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, ToSchema, Clone)]
pub struct DashboardStats {
    pub total_recipes: i32,
    pub public_recipes: i32,
    pub private_recipes: i32,
    pub total_views: i32,
}
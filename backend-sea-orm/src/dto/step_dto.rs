use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StepDto {
    pub id: Uuid,
    pub description: String,
    pub order: i32,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StepGroupDto {
    pub id: Uuid,
    pub position: i32,
    pub title: String,
    pub steps: Vec<StepDto>,
}
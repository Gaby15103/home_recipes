use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct IngredientDto {
    pub id: Uuid,
    pub name: String,
    pub quantity: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct IngredientGroupDto {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub position: i32,
    pub ingredients: Vec<IngredientDto>,
}
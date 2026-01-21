use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::dto::TagResponse;
use crate::models::{Ingredient, RecipeIngredient, Tag};
use crate::utils::unit::IngredientUnit;
use validator::Validate;

#[derive(Debug, Validate, Deserialize,Serialize)]
pub struct IngredientInput {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub quantity: BigDecimal,
    pub unit: IngredientUnit,
    pub note: Option<String>,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngredientResponse {
    pub id: Uuid,
    pub name: String,
    pub quantity: BigDecimal,
    pub unit: IngredientUnit,
    pub note: Option<String>,
    pub position: i32,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct IngredientUpdate {
    pub id: Option<Uuid>,
    pub name: String,
    pub quantity: BigDecimal,
    pub unit: IngredientUnit,
    pub note: Option<String>,
    pub position: i32,
}
impl From<(RecipeIngredient, Ingredient)> for IngredientResponse {
    fn from((ri, ing): (RecipeIngredient, Ingredient)) -> Self {
        Self {
            id: ing.id,
            name: ing.name,
            quantity: ri.quantity,
            unit: ri.unit.parse().unwrap(),
            note: ri.note,
            position: ri.position,
        }
    }
}
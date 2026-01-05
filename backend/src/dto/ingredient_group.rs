use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::dto::ingredient::{IngredientInput, IngredientResponse};

#[derive(Debug, Validate, Deserialize,Serialize)]
pub struct IngredientGroupInput {
    pub title: String,
    pub position: i32,
    pub ingredients: Vec<IngredientInput>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct IngredientGroupResponse {
    pub id: Uuid,
    pub title: String,
    pub position: i32,
    pub ingredients: Vec<IngredientResponse>,
}

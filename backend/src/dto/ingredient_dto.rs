use crate::dto::unit_dto::UnitDto;
use migration::prelude::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, Clone, ToSchema)]
pub struct IngredientTranslationInput {
    #[validate(length(min = 2, max = 5))]
    pub language_code: String,
    #[validate(length(min = 1, max = 50))]
    pub data: String,
    pub note: Option<String>,
}
#[derive(Debug, Validate, Deserialize, Serialize, Clone, ToSchema)]
pub struct EditIngredientTranslationInput {
    pub id: Option<Uuid>,
    #[validate(length(min = 2, max = 5))]
    pub language_code: String,
    #[validate(length(min = 1, max = 50))]
    pub data: String,
    pub note: Option<String>,
}
#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct IngredientInput {
    #[validate(nested)]
    pub translations: Vec<IngredientTranslationInput>,
    pub quantity: Decimal,
    pub unit_id: Uuid,
    pub position: i32,
}
#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct EditIngredientInput {
    pub id: Option<Uuid>,
    pub ingredient_id: Option<Uuid>,
    #[validate(nested)]
    pub translations: Vec<EditIngredientTranslationInput>,
    pub quantity: Decimal,
    pub unit_id: Uuid,
    pub position: i32,
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct IngredientRecipeViewDto {
    pub id: Uuid,
    pub ingredient_id: Uuid,
    pub name: String,
    pub unit: UnitDto,
    pub quantity: Decimal,
    pub note: Option<String>,
    pub position: i32,
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct IngredientViewDto {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct IngredientEditorDto {
    pub id: Uuid,
    pub ingredient_id: Uuid,
    pub quantity: Decimal,
    pub unit_id: Uuid,
    pub unit: UnitDto,
    pub position: i32,
    pub translations: Vec<IngredientTranslationsDto>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct IngredientTranslationsDto {
    pub id: Uuid,
    pub language_code: String,
    pub data: String,
    pub note: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct IngredientList{
    pub search: Option<String>,
    pub limit: i32,
}
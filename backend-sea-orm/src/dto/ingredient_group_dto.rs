use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use entity::{ingredient_group_translations, ingredient_groups, ingredients};
use crate::dto::ingredient_dto::{IngredientEditorDto, IngredientInput, IngredientViewDto};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct IngredientGroupInput {
    pub translations: Vec<IngredientGroupTranslationInput>,
    pub position: i32,
    #[validate(nested)]
    pub ingredients: Vec<IngredientInput>,
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone, ToSchema)]
pub struct IngredientGroupTranslationInput {
    pub language_code: String,
    #[validate(length(min = 1, max = 100))]
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct IngredientGroupViewDto {
    pub id: Uuid,
    pub name: String,
    pub recipe_id: Uuid,
    pub position: i32,
    pub ingredients: Vec<IngredientViewDto>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct IngredientGroupEditorDto {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub position: i32,
    pub ingredients: Vec<IngredientEditorDto>,
    pub translations: Vec<IngredientGroupTranslationDto>
}
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct IngredientGroupTranslationDto{
    pub language_code: String,
    pub title: String,
}
impl IngredientGroupViewDto {
    pub fn build(
        group: ingredient_groups::Model,
        name: String,
        ingredients: Vec<IngredientViewDto>
    ) -> Self {
        Self {
            id: group.id,
            name,
            recipe_id: group.recipe_id,
            position: group.position,
            ingredients,
        }
    }
}
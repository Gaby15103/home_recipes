use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::dto::ingredient_dto::{IngredientDto, IngredientGroupDto};
use crate::dto::step_dto::{StepDto, StepGroupDto};
use crate::dto::tag_dto::TagDto;
use crate::entities::{recipe_translations, recipes};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct RecipeDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RecipeTranslationDto {
    pub language_code: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RecipeResponseDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub servings: Option<i32>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub author_id: Uuid,
    pub author: String,
    pub is_private: bool,
    pub tags: Vec<TagDto>,
    pub ingredient_groups: Vec<Vec<IngredientGroupDto>>,
    pub step_groups: Vec<Vec<StepGroupDto>>,
}

impl From<(recipes::Model, recipe_translations::Model)> for RecipeResponseDto {
    fn from((recipe, translation): (recipes::Model, recipe_translations::Model)) -> Self {
        Self {
            id: recipe.id,
            title: translation.title,
            description: Option::from(translation.description),
            image_url: Option::from(recipe.image_url),
            servings: Option::from(recipe.servings),
            prep_time_minutes: Option::from(recipe.prep_time_minutes),
            cook_time_minutes: Option::from(recipe.cook_time_minutes),
            author_id: recipe.author_id.expect("REASON"),
            author: recipe.author,
            is_private: recipe.is_private,
            tags: vec![], // load from repository if needed
            ingredient_groups: vec![], // load from repository
            step_groups: vec![], // load from repository
        }
    }
}
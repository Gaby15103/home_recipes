use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::dto::tag_dto::{InputTag, TagDto};
use entity::{recipe_translations, recipes};
use utoipa::ToSchema;
use validator::Validate;
use crate::dto::ingredient_group_dto::{IngredientGroupInput, IngredientGroupViewDto};
use crate::dto::step_group_dto::{StepGroupEditorDto, StepGroupInput, StepGroupViewDto};

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
pub struct RecipeViewDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author_id: Uuid,
    pub author: String,
    pub is_private: bool,
    pub tags: Vec<TagDto>,
    pub ingredient_groups: Vec<IngredientGroupViewDto>,
    pub step_groups: Vec<StepGroupViewDto>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RecipeEditorDto {
    pub id: Uuid,
    pub translations: RecipeTranslationDto,
    pub image_url: Option<String>,
    pub servings: Option<i32>,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub author_id: Uuid,
    pub author: String,
    pub is_private: bool,
    pub tags: Vec<TagDto>,
    pub ingredient_groups: Vec<IngredientGroupViewDto>,
    pub step_groups: Vec<Vec<StepGroupEditorDto>>,
}
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct CreateRecipeInput {
    pub primary_language: String,
    #[validate(nested)]
    pub translations: Vec<RecipeTranslationInput>,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author_id: Uuid,
    pub author: String,
    pub is_private: bool,
    pub tags: Vec<InputTag>,
    #[validate(nested)]
    pub ingredient_groups: Vec<IngredientGroupInput>,
    #[validate(nested)]
    pub step_groups: Vec<StepGroupInput>,
}
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct RecipeTranslationInput {
    pub language_code: String,
    pub title: String,
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StepImageMeta {
    pub group_position: i32,
    pub step_position: i32,
    pub index: i32,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetAllRecipes {
    pub filters: GetFilter,
    pub include_private: bool,
    pub language_code: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetAllRecipesByPageQuery {
    pub filters: Option<GetFilter>,
    pub include_private: bool,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub lang: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetFilter {
    pub scope: Option<String>,

    pub search: Option<String>,
    pub ingredient: Option<String>,
    pub tags: Option<String>,

    pub min_prep: Option<i32>,
    pub max_prep: Option<i32>,
    pub min_cook: Option<i32>,
    pub max_cook: Option<i32>,

    pub min_steps: Option<i32>,
    pub max_steps: Option<i32>,

    pub date_from: Option<chrono::NaiveDate>,
    pub date_to: Option<chrono::NaiveDate>,
    pub lang: Option<String>,
}

impl From<(recipes::Model, recipe_translations::Model)> for RecipeViewDto {
    fn from((recipe, translation): (recipes::Model, recipe_translations::Model)) -> Self {
        Self {
            id: recipe.id,
            title: translation.title,
            description: translation.description,
            image_url: recipe.image_url,
            servings: recipe.servings,
            prep_time_minutes: recipe.prep_time_minutes,
            cook_time_minutes: recipe.cook_time_minutes,
            author_id: recipe.author_id.expect("REASON"),
            author: recipe.author,
            is_private: recipe.is_private,
            tags: vec![], // load from repository if needed
            ingredient_groups: vec![], // load from repository
            step_groups: vec![], // load from repository
        }
    }
}
impl RecipeViewDto {
    pub fn build(
        recipe: recipes::Model,
        translation: recipe_translations::Model,
        tags: Vec<TagDto>,
        ingredient_groups: Vec<IngredientGroupViewDto>,
        step_groups: Vec<StepGroupViewDto>,
    ) -> Self {
        Self {
            id: recipe.id,
            title: translation.title,
            description: translation.description,
            image_url: recipe.image_url,
            servings: recipe.servings,
            prep_time_minutes: recipe.prep_time_minutes,
            cook_time_minutes: recipe.cook_time_minutes,
            author_id: recipe.author_id.unwrap_or_default(),
            author: recipe.author,
            is_private: recipe.is_private,
            tags,
            ingredient_groups,
            step_groups,
        }
    }
}
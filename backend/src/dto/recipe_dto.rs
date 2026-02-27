use crate::dto::ingredient_group_dto::{EditIngredientGroupInput, IngredientGroupEditorDto, IngredientGroupInput, IngredientGroupViewDto};
use crate::dto::step_group_dto::{EditStepGroupInput, StepGroupEditorDto, StepGroupInput, StepGroupViewDto};
use crate::dto::tag_dto::{InputTag, TagDto};
use entity::{recipe_translations, recipes};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, ToSchema)]
pub struct RecipeDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RecipeTranslationDto {
    pub id: Uuid,
    pub language_code: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct RecipeViewDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author_id: Option<Uuid>,
    pub author: Option<String>,
    pub is_private: bool,
    pub tags: Vec<TagDto>,
    pub ingredient_groups: Vec<IngredientGroupViewDto>,
    pub step_groups: Vec<StepGroupViewDto>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[derive(Default)]
pub struct RecipeEditorDto {
    pub id: Uuid,
    pub primary_language: String,
    pub translations: Vec<RecipeTranslationDto>,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author_id: Option<Uuid>,
    pub author: Option<String>,
    pub is_private: bool,
    pub tags: Vec<TagDto>,
    pub ingredient_groups: Vec<IngredientGroupEditorDto>,
    pub step_groups: Vec<StepGroupEditorDto>,
}
#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct CreateRecipeInput {
    pub primary_language: String,
    #[validate(nested)]
    pub translations: Vec<RecipeTranslationInput>,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author_id: Option<Uuid>,
    pub author: Option<String>,
    pub is_private: bool,
    pub tags: Vec<InputTag>,
    #[validate(nested)]
    pub ingredient_groups: Vec<IngredientGroupInput>,
    #[validate(nested)]
    pub step_groups: Vec<StepGroupInput>,
}
#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct EditRecipeInput {
    pub id: Uuid,
    pub primary_language: String,
    #[validate(nested)]
    pub translations: Vec<EditRecipeTranslationInput>,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author_id: Option<Uuid>,
    pub author: Option<String>,
    pub is_private: bool,
    pub tags: Vec<InputTag>,
    #[validate(nested)]
    pub ingredient_groups: Vec<EditIngredientGroupInput>,
    #[validate(nested)]
    pub step_groups: Vec<EditStepGroupInput>,
}
#[derive(Debug, Serialize, Deserialize, Validate, Clone, ToSchema)]
pub struct RecipeTranslationInput {
    pub language_code: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize, Validate, Clone, ToSchema)]
pub struct EditRecipeTranslationInput {
    pub id: Option<Uuid>,
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
    pub filters: RecipeFilter,
    pub include_private: bool,
    pub language_code: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetAllRecipesByPageQuery {
    pub filters: Option<RecipeFilter>,
    pub include_private: bool,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub lang: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct RecipeFilter {
    #[serde(default)]
    pub scope: bool,

    pub search: Option<String>,
    pub ingredient: Option<Vec<String>>,
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

#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct RecipeFilterByPage{
    pub filters: Option<RecipeFilter>,
    pub page:  Option<i32>,
    pub per_page: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct RecipePagination{
    pub data: Vec<RecipeViewDto>,
    pub total: i32,
    pub page: i32,
    pub per_page: i32,
}
#[derive(Deserialize)]
pub struct GetRecipeQuery {
    pub include_translations: Option<bool>,
}
#[derive(Deserialize)]
pub struct LastRecipesQuery {
    pub nb: Option<i64>,
    pub include_translations: Option<bool>,
}
#[derive(Serialize, ToSchema)]
#[serde(untagged)]
pub enum RecipeResponse {
    View(RecipeViewDto),
    Editor(RecipeEditorDto),
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
            author_id: recipe.author_id,
            author: recipe.author,
            is_private: recipe.is_private,
            tags: vec![],
            ingredient_groups: vec![],
            step_groups: vec![],
        }
    }
}
impl From<recipe_translations::Model> for RecipeTranslationDto {
    fn from(model: recipe_translations::Model) -> Self {
        Self {
            id: model.id,
            language_code: model.language_code,
            title: model.title,
            description: Option::from(model.description),
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
            author_id: recipe.author_id,
            author: recipe.author,
            is_private: recipe.is_private,
            tags,
            ingredient_groups,
            step_groups,
        }
    }
}
impl RecipeEditorDto {
    pub fn build_full(
        recipe: recipes::Model,
        translation: Vec<RecipeTranslationDto>,
        tags: Vec<TagDto>,
        ingredient_groups: Vec<IngredientGroupEditorDto>,
        step_groups: Vec<StepGroupEditorDto>,
    ) -> Self {
        Self {
            id: recipe.id,
            primary_language: recipe.original_language_code,
            translations: translation,
            image_url: recipe.image_url,
            servings: recipe.servings,
            prep_time_minutes: recipe.prep_time_minutes,
            cook_time_minutes: recipe.cook_time_minutes,
            author_id: recipe.author_id,
            author: recipe.author,
            is_private: recipe.is_private,
            tags,
            ingredient_groups,
            step_groups,
        }
    }
}
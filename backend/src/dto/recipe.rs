use crate::app::recipes::In;
use crate::dto::{
    IngredientGroupUpdate, InputTag, StepGroupResponse, StepGroupUpdate,
    ingredient_group::{IngredientGroupInput, IngredientGroupResponse},
    step::StepGroupInput,
    tag::TagResponse,
};
use crate::models::Recipe;
use crate::utils::auth::Auth;
use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct CreateRecipeInput {
    pub title: String,
    pub description: Option<String>,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
    pub tags: Vec<InputTag>,
    pub ingredient_groups: Vec<IngredientGroupInput>,
    pub step_groups: Vec<StepGroupInput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
    pub tags: Vec<TagResponse>,
    pub ingredient_groups: Vec<IngredientGroupResponse>,
    pub step_groups: Vec<StepGroupResponse>,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UpdateRecipeInput {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
    pub tags: Vec<InputTag>,
    pub ingredient_groups: Vec<IngredientGroupUpdate>,
    pub step_groups: Vec<StepGroupUpdate>,
}

pub struct CreateRecipe {
    pub auth: Auth,
    pub new_recipe: CreateRecipeInput,
    pub main_image: TempFile,
    pub step_images: Vec<TempFile>,
    pub step_images_meta: Vec<StepImageMeta>,
}

pub struct UpdateRecipe {
    pub auth: Auth,
    pub update_recipe: UpdateRecipeInput,
    pub main_image: Option<TempFile>,
    pub step_images: Vec<TempFile>,
    pub step_images_meta: Vec<StepImageMeta>,
}

pub struct GetRecipeById {
    pub id: Uuid,
}

#[derive(Debug, MultipartForm)]
pub struct CreateRecipeForm {
    pub recipe: MpJson<In<CreateRecipeInput>>,
    pub main_image: TempFile,
    pub step_images: Vec<TempFile>,
    pub step_images_meta: MpJson<Vec<StepImageMeta>>,
}

#[derive(Debug, MultipartForm)]
pub struct UpdateRecipeForm {
    pub recipe: MpJson<In<UpdateRecipeInput>>,
    pub main_image: Option<TempFile>,
    pub step_images: Vec<TempFile>,
    pub step_images_meta: MpJson<Vec<StepImageMeta>>,
}

#[derive(Debug, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct StepImageMeta {
    pub group_position: usize,
    pub step_position: usize,
    pub index: usize,
}

pub struct DeleteRecipe {
    pub recipe_id: Uuid,
}

impl RecipeResponse {
    pub fn from_parts(
        recipe: Recipe,
        tags: Vec<TagResponse>,
        ingredient_groups: Vec<IngredientGroupResponse>,
        step_groups: Vec<StepGroupResponse>,
    ) -> Self {
        Self {
            id: recipe.id,
            title: recipe.title,
            description: recipe.description,
            image_url: recipe.image_url,
            servings: recipe.servings,
            prep_time_minutes: recipe.prep_time_minutes,
            cook_time_minutes: recipe.cook_time_minutes,
            author: recipe.author,
            author_id: recipe.author_id,
            is_private: recipe.is_private,
            tags,
            ingredient_groups,
            step_groups,
        }
    }
}

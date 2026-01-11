use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::dto::{tag::TagResponse, ingredient_group::{IngredientGroupInput, IngredientGroupResponse}, step::StepGroupInput, StepGroupResponse, InputTag, IngredientGroupUpdate, StepGroupUpdate};
use crate::models::{IngredientGroup, Recipe, StepGroup, Tag};

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

#[derive(Debug, Serialize)]
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

use actix_web::{HttpResponse, web::{Data, Json}, HttpRequest};
use validator::Validate;

use super::AppState;
use crate::models::{Ingredient, Recipe, Tag};
use crate::prelude::*;
use actix::Message;
use bigdecimal::BigDecimal;
use diesel::sql_types::Array;
use uuid::Uuid;
use crate::app::tags::InputTag;
use crate::utils::auth::{authenticate, Auth};
use crate::utils::unit::IngredientUnit;

#[derive(Debug, Deserialize)]
pub struct In<U> {
    recipe: U,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRecipe {
    #[validate(length(min = 1, max = 32))]
    pub title: String,
    pub description: Option<String>,
    #[validate(range(min = 1))]
    pub servings: i32,
    #[validate(range(min = 1))]
    pub prep_time_minutes: i32,
    #[validate(range(min = 1))]
    pub cook_time_minutes: i32,
    #[validate(length(min = 1, max = 32))]
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
    pub tags: Vec<InputTag>,
    pub ingredient_groups: Vec<IngredientGroupInput>,
    pub step_groups: Vec<StepGroupInput>
}
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IngredientGroupInput {
    #[validate(length(min = 1, max = 50))]
    pub title: String,
    pub position: i32,
    pub ingredients: Vec<IngredientInput>
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IngredientInput {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub quantity: BigDecimal,
    pub unit: IngredientUnit,
    pub note: Option<String>,
    pub position: i32
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepGroupInput {
    #[validate(length(min = 1, max = 50))]
    pub title: String,
    pub position: i32,
    pub steps: Vec<StepInput>
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepInput {
    pub position: i32,
    #[validate(length(min = 1))]
    pub instruction: String,
    pub duration_minutes: i32
}

#[derive(Debug)]
pub struct CreateRecipeOuter {
    pub auth: Auth,
    pub new_recipe: CreateRecipe,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateRecipe {
    pub id: Uuid,
    #[validate(length(min = 1, max = 32))]
    pub title: String,
    pub description: Option<String>,
    #[validate(range(min = 1))]
    pub servings: i32,
    #[validate(range(min = 1))]
    pub prep_time_minutes: i32,
    #[validate(range(min = 1))]
    pub cook_time_minutes: i32,
    #[validate(length(min = 1, max = 32))]
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
}
#[derive(Debug)]
pub struct UpdateRecipeOuter {
    pub auth: Auth,
    pub update_recipe: UpdateRecipe,
}

#[derive(Debug, Serialize)]
pub struct RecipeResponse {
    pub recipe: RecipeResponseInner,
}

#[derive(Debug, Serialize)]
pub struct RecipeResponseInner {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
}


impl From<Recipe> for RecipeResponse {
    fn from(recipe: Recipe) -> Self {
        RecipeResponse {
            recipe: RecipeResponseInner{
                id: recipe.id,
                title: recipe.title,
                description: recipe.description,
                servings: recipe.servings,
                prep_time_minutes: recipe.prep_time_minutes,
                cook_time_minutes: recipe.cook_time_minutes,
                author: recipe.author,
                author_id: recipe.author_id,
                is_private: recipe.is_private,
            }
        }
    }
}

pub async fn create(
    state: Data<AppState>,
    (form, req): (Json<In<CreateRecipe>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let new_recipe = form.into_inner().recipe;

    new_recipe.validate()?;

    let auth = authenticate(&state, &req).await?;

    let res = state
        .db
        .send(CreateRecipeOuter { auth, new_recipe })
        .await
        .map_err(|_| crate::error::Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    state: Data<AppState>,
    (form, req): (Json<In<UpdateRecipe>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let update_recipe = form.into_inner().recipe;

    update_recipe.validate()?;

    let auth = authenticate(&state, &req).await?;

    let res = state
        .db
        .send(UpdateRecipeOuter { auth, update_recipe })
        .await
        .map_err(|_| crate::error::Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}
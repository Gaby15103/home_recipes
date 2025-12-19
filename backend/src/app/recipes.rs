use actix_web::{HttpResponse, web::{Data, Json}, HttpRequest};
use validator::Validate;

use super::AppState;
use crate::prelude::*;
use crate::utils::auth::{authenticate, Auth};
use crate::dto::*;
use crate::schema::recipes::dsl::recipes;

#[derive(Debug, Deserialize)]
pub struct In<U> {
    recipe: U,
}

pub struct CreateRecipe {
    pub auth: Auth,
    pub new_recipe: CreateRecipeInput,
}

pub struct UpdateRecipe {
    pub auth: Auth,
    pub update_recipe: UpdateRecipeInput,
}
pub struct GetAllRecipes;

pub async fn create(
    state: Data<AppState>,
    (form, req): (Json<In<CreateRecipeInput>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let new_recipe = form.into_inner().recipe;

    new_recipe.validate()?;

    let auth = authenticate(&state, &req).await?;

    let res = state
        .db
        .send(CreateRecipe { auth, new_recipe })
        .await
        .map_err(|_| crate::error::Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    state: Data<AppState>,
    (form, req): (Json<In<UpdateRecipeInput>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let update_recipe = form.into_inner().recipe;

    update_recipe.validate()?;

    let auth = authenticate(&state, &req).await?;

    let res = state
        .db
        .send(UpdateRecipe { auth, update_recipe })
        .await
        .map_err(|_| crate::error::Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_all(
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let res = state
        .db
        .send(GetAllRecipes)
        .await
        .map_err(|_| crate::error::Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}


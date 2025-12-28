use std::collections::HashMap;
use actix_web::{HttpResponse, web::{Data, Json}, HttpRequest, web};
use validator::Validate;

use super::AppState;
use crate::prelude::*;
use crate::utils::auth::{authenticate, Auth};
use crate::dto::*;
use crate::schema::recipes::dsl::recipes;
use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use crate::models::Role;

#[derive(MultipartForm)]
pub struct CreateRecipeForm {
    pub recipe: MpJson<In<CreateRecipeInput>>,
    pub main_image: TempFile,
    pub step_image: Vec<TempFile>,
}

#[derive(Debug, Deserialize)]
pub struct In<U> {
    recipe: U,
}

pub struct CreateRecipe {
    pub auth: Auth,
    pub new_recipe: CreateRecipeInput,
    pub main_image: TempFile,
    pub step_image: Vec<TempFile>,
}

pub struct UpdateRecipe {
    pub auth: Auth,
    pub update_recipe: UpdateRecipeInput,
}
pub struct GetAllRecipes{
 pub private: bool,
}

pub async fn create(
    state: Data<AppState>,
    req: HttpRequest,
    MultipartForm(form): MultipartForm<CreateRecipeForm>,
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;

    let new_recipe = form.recipe.into_inner().recipe;

    new_recipe.validate()?;

    let res = state
        .db
        .send(CreateRecipe { auth, new_recipe, main_image: form.main_image, step_image: form.step_image })
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

#[derive(Debug, Deserialize)]
pub struct GetFilter{
    private: Option<bool>,
}

pub async fn get_all(
    state: Data<AppState>,
    query: web::Query<GetFilter>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut res: Vec<RecipeResponse> = Vec::new();
    if query.private.unwrap_or(false) {
        let auth = authenticate(&state, &req).await?;
        if auth.roles.iter().any(|r| r.name == "ADMIN" || r.name == "MODERATOR") {
            res = state
                .db
                .send(GetAllRecipes {private: query.private.unwrap_or(false)})
                .await
                .map_err(|_| crate::error::Error::InternalServerError)??;
        } else {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    }else{
        res = state
            .db
            .send(GetAllRecipes {private: query.private.unwrap_or(false)})
            .await
            .map_err(|_| crate::error::Error::InternalServerError)??;
    }

    Ok(HttpResponse::Ok().json(res))
}


use actix_web::{HttpResponse, web::{Data, Json}, HttpRequest, web};
use validator::Validate;

use super::AppState;
use crate::prelude::*;
use crate::utils::auth::{authenticate, Auth};
use crate::dto::*;
use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use uuid::Uuid;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct StepImageMeta {
    pub group_position: usize,
    pub step_position: usize,
    pub index: usize,
}


#[derive(Debug, Deserialize)]
pub struct In<U> {
    recipe: U,
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
pub async fn get_by_id(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let id_str = path.into_inner();
    let id = Uuid::parse_str(&id_str).map_err(|_| Error::InternalServerError)?;

    let recipe = state
        .db
        .send(GetRecipeById { id })
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(recipe))
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
        .send(CreateRecipe {
            auth,
            new_recipe,
            main_image: form.main_image,
            step_images: form.step_images,
            step_images_meta: form.step_images_meta.into_inner(),
        })
        .await
        .map_err(|_| crate::error::Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    state: Data<AppState>,
    req: HttpRequest,
    MultipartForm(form): MultipartForm<UpdateRecipeForm>,
) -> Result<HttpResponse, Error> {
    let update_recipe = form.recipe.into_inner().recipe;

    update_recipe.validate()?;

    let auth = authenticate(&state, &req).await?;

    let res = state
        .db
        .send(UpdateRecipe {
            auth,
            update_recipe,
            main_image: form.main_image,
            step_images: form.step_images,
            step_images_meta: form.step_images_meta.into_inner(),
        })
        .await
        .map_err(|_| crate::error::Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug, Deserialize)]
pub struct GetFilter{
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
}

pub struct GetAllRecipes {
    pub filters: GetFilter,
    pub include_private: bool,
}


pub async fn get_all(
    state: Data<AppState>,
    query: web::Query<GetFilter>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {

    let is_admin_scope = matches!(query.scope.as_deref(), Some("true"));

    let include_private = if is_admin_scope {
        let auth = authenticate(&state, &req).await?;

        let allowed = auth.roles.iter().any(|r|
            r.name == "ADMIN" || r.name == "MODERATOR"
        );

        if !allowed {
            return Ok(HttpResponse::Unauthorized().finish());
        }

        true
    } else {
        false
    };

    let recipes = state
        .db
        .send(GetAllRecipes {
            filters: query.into_inner(),
            include_private,
        })
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(recipes))
}

#[derive(Debug, Deserialize)]
pub struct GetAllRecipesByPage {
    pub filters: Option<GetFilter>,
    pub include_private: bool,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedRecipes {
    pub data: Vec<RecipeResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

pub async fn get_by_page(
    state: Data<AppState>,
    query: web::Query<GetAllRecipesByPage>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let query = query.into_inner();

    let include_private = if query.include_private {
        let auth = authenticate(&state, &req).await?;
        let allowed = auth.roles.iter().any(|r| r.name == "ADMIN" || r.name == "MODERATOR");
        if !allowed {
            return Ok(HttpResponse::Unauthorized().finish());
        }
        true
    } else {
        false
    };

    let recipes = state
        .db
        .send(GetAllRecipesByPage {
            filters: query.filters,
            include_private,
            page: query.page,
            per_page: query.per_page,
        })
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(recipes))
}



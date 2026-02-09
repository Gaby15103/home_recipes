use std::ops::Deref;
use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;
use crate::app::state::AppState;
use crate::dto::recipe_dto::RecipeResponseDto;
use crate::errors::Error;

use crate::services::recipe_service;
use crate::utils::header_extractor::extract_language;

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/recipes")
            .route("", web::get().to(list))
            .route("/{id}", web::get().to(get))
    );
}

/// List all recipes
#[utoipa::path(
    get,
    path = "/recipes",
    responses(
        (status = 200, description = "List all recipes", body = [RecipeResponseDto])
    )
)]
pub async fn list(
    state: web::Data<AppState>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let lang_code = extract_language(&req);

    let recipes = recipe_service::get_all(&state.db, lang_code.deref()).await?;

    Ok(HttpResponse::Ok().json(recipes))
}
/// Get one recipe by ID
#[utoipa::path(
    get,
    path = "/recipes/{id}",
    params(
        ("id" = Uuid, Path, description = "Recipe ID")
    ),
    responses(
        (status = 200, description = "Get recipe by ID", body = RecipeResponseDto),
        (status = 404, description = "Recipe not found")
    )
)]
pub async fn get(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let lang_code = extract_language(&req);

    let recipe = recipe_service::get_by_id(&state.db, id.into_inner(),lang_code.deref()).await?;

    Ok(HttpResponse::Ok().json(recipe))
}

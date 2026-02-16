use std::ops::Deref;
use actix_multipart::form::{MultipartForm};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::http::header::q;
use actix_web::web::Query;
use uuid::Uuid;
use crate::app::state::AppState;
use crate::domain::user::{AuthenticatedUser, Role};
use actix_web::web::Json;
use sea_orm::sqlx::query;
use crate::dto::auth_dto::LoginRequestDto;
use crate::dto::recipe_dto::{CreateRecipeInput, GetAllRecipesByPageQuery, RecipeFilter, RecipeFilterByPage, RecipeViewDto};
use crate::errors::Error;

use crate::services::recipe_service;
use crate::utils::header_extractor::extract_language;

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/recipes")
            .route("", web::get().to(list))
            .route("/by_page", web::get().to(get_by_page))
            .route("", web::post().to(create))
            .route("/favorites", web::get().to(get_favorites))
            .route("/{id}", web::get().to(get))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete))
            .route("/{id}/analytics", web::get().to(analytics))
            .route("/{id}/views", web::post().to(track_view))
            .route("/{id}/favorite", web::post().to(favorite))
            .route("/{id}/rating", web::post().to(rate))
            .route("/{id}/rating", web::delete().to(unrate))
            .route("/{id}/rating", web::get().to(get_rating))
            .route("/{id}/comments", web::get().to(get_comments))
            .route("/{id}/comments", web::post().to(add_comment))
            .route("/{recipe_id}/versions/{version_id}/restore", web::post().to(restore_version), )
    );
}

/// List all recipes
#[utoipa::path(
    get,
    path = "/recipes",
    responses(
        (status = 200, description = "List all recipes", body = [RecipeViewDto])
    )
)]
pub async fn list(
    state: web::Data<AppState>,
    req: HttpRequest,
    query: web::Query<RecipeFilter>,
    auth: Option<AuthenticatedUser>,
) -> Result<HttpResponse, Error> {
    if query.scope && auth.is_some() {
        auth.unwrap().require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;
    }
    let lang_code = extract_language(&req);

    let recipes = recipe_service::get_all(&state.db, lang_code.deref(), query.into_inner()).await?;

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
        (status = 200, description = "Get recipe by ID", body = RecipeViewDto),
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

pub async fn get_by_page(
    state: web::Data<AppState>,
    req: HttpRequest,
    query: web::Query<RecipeFilterByPage>,
    auth: Option<AuthenticatedUser>,
) -> Result<HttpResponse, Error> {
    if let Some(filters) = &query.filters{
        if filters.scope && auth.is_some() {
            auth.unwrap().require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;
        }
    }

    let lang_code = extract_language(&req);

    let recipes = recipe_service::get_all_by_page(&state.db, lang_code.deref(), query.into_inner()).await?;

    Ok(HttpResponse::Ok().json(recipes))
}
pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    auth: AuthenticatedUser,
    input: Json<CreateRecipeInput>,
) -> Result<HttpResponse, Error> {
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;
    let new_recipe = input.into_inner();
    let res: RecipeViewDto = recipe_service::create(&state.db, new_recipe, auth.user.preferences.language.unwrap().deref()).await?;

    Ok(HttpResponse::Ok().json(res))
}
pub async fn get_favorites(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn update(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn delete(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn analytics(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn track_view(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn favorite(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn rate(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn unrate(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn get_rating(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn get_comments(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn add_comment(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn restore_version(
    state: web::Data<AppState>,
    query: Query<GetAllRecipesByPageQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
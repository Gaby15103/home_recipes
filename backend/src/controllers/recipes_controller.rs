use crate::app::state::AppState;
use crate::domain::user::{AuthenticatedUser, Role};
use crate::dto::comment_dto::{CommentDto, CreateCommentDto};
use crate::dto::recipe_dto::{CreateRecipeInput, EditRecipeInput, GetRecipeQuery, LastRecipesQuery, RecipeFilter, RecipeFilterByPage, RecipePagination, RecipeResponse, RecipeViewDto};
use crate::dto::recipe_rating_dto::RecipeRatingDto;
use crate::errors::Error;
use actix_web::web::{Json, Path};
use actix_web::web::{Data, Query};
use actix_web::{web, HttpRequest, HttpResponse};
use std::ops::Deref;
use uuid::Uuid;
use validator::Validate;

use crate::services::{recipe_service, user_service};
use crate::utils::header_extractor::extract_language;

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/recipes")
            .route("", web::get().to(list))
            .route("/last", web::get().to(get_last))
            .route("/by_page", web::get().to(get_by_page))
            .route("", web::post().to(create))
            .route("/favorites", web::get().to(get_favorites))
            .route("/comment/{id}", web::delete().to(delete_comment))
            .route("/comment/{id}", web::put().to(edit_comment))
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
            .route("/{recipe_id}/versions/", web::get().to(get_versions))
            .route("/{recipe_id}/versions/{version_id}", web::get().to(get_version))
            .route("/{recipe_id}/versions/{version_id}/restore", web::post().to(restore_version))
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
    state: Data<AppState>,
    req: HttpRequest,
    query: Query<RecipeFilter>,
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
    state:Data<AppState>,
    id: Path<Uuid>,
    query: Query<GetRecipeQuery>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {

    let include_translations = query.include_translations;

    let lang_code = extract_language(&req);

    let recipe = recipe_service::get_by_id(&state.db, id.into_inner(),lang_code.deref(),include_translations.unwrap_or_else(|| false)).await?;
    match recipe {
        RecipeResponse::View(view_data) => {
            Ok(HttpResponse::Ok().json(view_data))
        }
        RecipeResponse::Editor(view_data) => {
            Ok(HttpResponse::Ok().json(view_data))
        }
    }
}

/// Get the latest recipes
#[utoipa::path(
    get,
    path = "/recipes/last",
    params(
        ("nb" = Option<i64>, Query, description = "Number of recipes to fetch"),
        ("include_translations" = Option<bool>, Query, description = "Whether to include all translations")
    ),
    responses(
        (status = 200, description = "List of latest recipes", body = [RecipeViewDto])
    )
)]
pub async fn get_last(
    state: Data<AppState>,
    req: HttpRequest,
    query: Query<LastRecipesQuery>,
) -> Result<HttpResponse, Error> {
    let lang_code = extract_language(&req);

    // Default to 4 if nb is not provided, capped at 20 for safety
    let limit = query.nb.unwrap_or(4).min(20);
    let include_translations = query.include_translations.unwrap_or(false);

    // Call the service layer (you will need to implement this in recipe_service)
    let recipes = recipe_service::get_last(
        &state.db,
        lang_code.deref(),
        limit,
        include_translations
    ).await?;

    Ok(HttpResponse::Ok().json(recipes))
}

pub async fn get_by_page(
    state: Data<AppState>,
    req: HttpRequest,
    query: Query<RecipeFilterByPage>,
    auth: Option<AuthenticatedUser>,
) -> Result<HttpResponse, Error> {
    if let Some(filters) = &query.filters{
        if filters.scope && auth.is_some() {
            auth.unwrap().require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;
        }
    }

    let lang_code = extract_language(&req);
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    let recipes = recipe_service::get_all_by_page(&state.db, lang_code.deref(), query.into_inner().clone()).await?;

    Ok(HttpResponse::Ok().json(RecipePagination {
        data: recipes.clone(),
        total: recipes.len() as i32,
        page,
        per_page,
    }))
}
pub async fn create(
    state: Data<AppState>,
    req: HttpRequest,
    auth: AuthenticatedUser,
    input: Json<CreateRecipeInput>,
) -> Result<HttpResponse, Error> {
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;
    let new_recipe = input.into_inner();
    let lang_code = extract_language(&req);
    let res: RecipeViewDto = recipe_service::create(&state.db, new_recipe, lang_code.deref()).await?;

    Ok(HttpResponse::Ok().json(res))
}
pub async fn get_favorites(
    state: Data<AppState>,
    auth: Option<AuthenticatedUser>,
) -> Result<HttpResponse, Error> {
    if let Some(auth) = auth {
        let favorites = user_service::get_favorites(&state.db, auth.user).await?;
        return Ok(HttpResponse::Ok().json(favorites));
    }

    Ok(HttpResponse::NoContent().finish())
}
pub async fn update(
    state: Data<AppState>,
    req: HttpRequest,
    auth: AuthenticatedUser,
    input: Json<EditRecipeInput>,
    path: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;

    let updated_recipe = input.into_inner();

    updated_recipe.validate()?;

    let recipe_id = path.into_inner();

    let lang_code = extract_language(&req);

    let result = recipe_service::update(&state.db, updated_recipe, recipe_id, lang_code.deref(), auth.user).await?;

    Ok(HttpResponse::Ok().json(result))
}
pub async fn delete(
    state: Data<AppState>,
    path: Path<Uuid>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;
    let recipe_id = path.into_inner();
    if !recipe_service::delete(&state.db, recipe_id).await?{
        return Ok(HttpResponse::NotFound().finish());
    }
    Ok(HttpResponse::Ok().finish())
}
pub async fn analytics(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();
    let count = recipe_service::analytics(&state.db, recipe_id).await?;
    Ok(HttpResponse::Ok().json(count))
}
pub async fn track_view(
    state: Data<AppState>,
    path: Path<Uuid>,
    auth: Option<AuthenticatedUser>,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();
    let user_id = auth.map(|a| a.user.id);
    recipe_service::add_view(&state.db, recipe_id, user_id).await?;

    Ok(HttpResponse::Ok().finish())
}
pub async fn favorite(
    state: Data<AppState>,
    path: Path<Uuid>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();
    let user_id = auth.user.id.clone();
    let favorited = recipe_service::toogle_favorite(&state.db, recipe_id, user_id).await?;
    Ok(HttpResponse::Ok().json(favorited))
}
pub async fn rate(
    state: Data<AppState>,
    path: Path<Uuid>,
    auth: AuthenticatedUser,
    body: Json<i32>,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();
    let user_id = auth.user.id.clone();
    let rating = body.into_inner();
    recipe_service::rate(&state.db,recipe_id, user_id, rating).await?;
    Ok(HttpResponse::Ok().json({}))
}
pub async fn unrate(
    state: Data<AppState>,
    path: Path<Uuid>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();
    let user_id = auth.user.id.clone();
    recipe_service::unrate(&state.db,recipe_id, user_id).await?;
    Ok(HttpResponse::Ok().json({}))
}
pub async fn get_rating(
    state: Data<AppState>,
    path: Path<Uuid>,
    auth: Option<AuthenticatedUser>,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();
    let user_id = auth.map(|a| a.user.id);
    let rating:RecipeRatingDto = recipe_service::get_rating(&state.db, recipe_id, user_id).await?;
    Ok(HttpResponse::Ok().json(rating))
}
pub async fn get_comments(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();
    let comments = recipe_service::get_comments(&state.db, recipe_id).await?;
    Ok(HttpResponse::Ok().json(comments))
}
pub async fn add_comment(
    state: Data<AppState>,
    path: Path<Uuid>,
    body: Json<CreateCommentDto>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let new_comment = body.into_inner();
    let recipe_id = path.into_inner();
    let user_id = auth.user.id.clone();
    let comment = recipe_service::add_comment(&state.db, new_comment, recipe_id, user_id).await?;
    Ok(HttpResponse::Ok().json(comment))
}
pub async fn delete_comment(
    state: Data<AppState>,
    path: Path<Uuid>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let comment_id = path.into_inner();
    let comment = recipe_service::delete_comment(&state.db, comment_id, auth).await?;
    Ok(HttpResponse::Ok().json(comment))
}
pub async fn edit_comment(
    state: Data<AppState>,
    path: Path<Uuid>,
    body: Json<CommentDto>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let comment_id = path.into_inner();
    let edit_comment = body.into_inner();
    let comment = recipe_service::edit_comment(&state.db, comment_id, auth,edit_comment).await?;
    Ok(HttpResponse::Ok().json(comment))
}
pub async fn get_versions(
    state: Data<AppState>,
    path: Path<Uuid>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;
    let versions = recipe_service::get_versions(&state.db, recipe_id).await?;
    Ok(HttpResponse::Ok().json(versions))
}
pub async fn get_version(
    state: Data<AppState>,
    path: Path<(Uuid, Uuid)>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let (recipe_id, version_id) = path.into_inner();
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;
    let version = recipe_service::get_version(&state.db, recipe_id,version_id).await?;
    Ok(HttpResponse::Ok().json(version))
}
pub async fn restore_version(
    path: Path<(Uuid, Uuid)>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let (recipe_id, version_id) = path.into_inner();
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;
    Ok(HttpResponse::Ok().json({}))
}
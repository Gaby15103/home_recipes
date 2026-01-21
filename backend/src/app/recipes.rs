use actix_web::{
    HttpRequest, HttpResponse, web,
    web::{Data},
};
use validator::Validate;

use super::AppState;
use crate::dto::*;
use crate::prelude::*;
use crate::utils::auth::{authenticate};
use actix_multipart::form::{MultipartForm};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct In<U> {
    recipe: U,
}

pub async fn get(
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
pub struct GetFilter {
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

pub async fn list(
    state: Data<AppState>,
    query: web::Query<GetFilter>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let is_admin_scope = matches!(query.scope.as_deref(), Some("true"));

    let include_private = if is_admin_scope {
        let auth = authenticate(&state, &req).await?;

        let allowed = auth
            .roles
            .iter()
            .any(|r| r.name == "ADMIN" || r.name == "MODERATOR");

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
        let allowed = auth
            .roles
            .iter()
            .any(|r| r.name == "ADMIN" || r.name == "MODERATOR");
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

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    let recipe_id = path.into_inner();

    state
        .db
        .send(GetRecipeById { id: recipe_id })
        .await??;

    let is_admin = auth.roles.iter().any(|r| {
        r.name == "ADMIN" || r.name == "MODERATOR"
    });

    if !is_admin {
        return Err(Error::Forbidden(serde_json::json!({
            "error": "You are not allowed to delete this recipe"
        })));
    }

    state
        .db
        .send(DeleteRecipe { recipe_id })
        .await??;

    Ok(HttpResponse::NoContent().finish())
}


pub async fn analytics(
    state: Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();

    let count = state.db.send(GetRecipeAnalytics { recipe_id }).await??;

    Ok(HttpResponse::Ok().json(count))
}

pub async fn track_view(
    state: Data<AppState>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();
    let auth = authenticate(&state, &req).await.ok();

    state
        .db
        .send(RegisterRecipeView {
            recipe_id,
            user_id: auth.map(|a| a.user.id),
        })
        .await??;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn favorite(
    state: Data<AppState>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    let recipe_id = path.into_inner();

    let favorited = state
        .db
        .send(ToggleFavorite {
            user_id: auth.user.id,
            recipe_id,
        })
        .await??;

    Ok(HttpResponse::Ok().json(favorited))
}

pub async fn get_favorites(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;

    let recipes = state
        .db
        .send(GetFavoriteRecipes {
            user_id: auth.user.id,
        })
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(recipes))
}

pub async fn rate(
    state: Data<AppState>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<i32>,
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    let recipe_id = path.into_inner();

    state
        .db
        .send(SetRecipeRating {
            recipe_id,
            user_id: auth.user.id,
            rating: body.into_inner(),
        })
        .await??;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_rating(
    state: Data<AppState>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();
    let auth = authenticate(&state, &req).await.ok();

    let rating = state
        .db
        .send(GetRecipeRating {
            recipe_id,
            user_id: auth.map(|a| a.user.id),
        })
        .await??;

    Ok(HttpResponse::Ok().json(rating))
}

pub async fn unrate(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    let recipe_id = path.into_inner();

    state
        .db
        .send(UnsetRecipeRating {
            recipe_id,
            user_id: auth.user.id,
        })
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn add_comment(
    state: Data<AppState>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<CreateComment>,
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;

    let mut cmd = body.into_inner();
    cmd.recipe_id = path.into_inner();
    cmd.user_id = auth.user.id;

    let comment = state.db.send(cmd).await??;
    Ok(HttpResponse::Created().json(comment))
}

pub async fn get_comments(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();

    let comments = state
        .db
        .send(GetRecipeComments { recipe_id })
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(comments))
}

pub async fn delete_comment(
    state: Data<AppState>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    authenticate(&state, &req).await?;

    state
        .db
        .send(DeleteComment {
            comment_id: path.into_inner(),
        })
        .await??;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_versions(
    state: Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let recipe_id = path.into_inner();

    let versions = state.db.send(GetRecipeVersions { recipe_id }).await??;

    Ok(HttpResponse::Ok().json(versions))
}

pub async fn get_version(
    state: Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let version = state.db.send(GetRecipeVersion { id }).await??;

    Ok(HttpResponse::Ok().json(version))
}

pub async fn restore_version(
    state: Data<AppState>,
    req: HttpRequest,
    path: web::Path<(Uuid, Uuid)>, // (recipe_id, version_id)
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;

    let (recipe_id, version_id) = path.into_inner();

    let recipe = state
        .db
        .send(RestoreRecipeVersion {
            recipe_id,
            version_id,
            user_id: auth.user.id,
        })
        .await??;

    Ok(HttpResponse::Ok().json(recipe))
}

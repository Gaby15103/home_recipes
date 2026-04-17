use crate::app::state::AppState;
use crate::domain::user::AuthenticatedUser;
use crate::dto::recipe_dto::{GetAllRecipesByPageQuery, LastRecipesQuery, RecipeFilter};
use crate::dto::studio_dto::DashboardStats;
use crate::errors::Error;
use crate::services::{recipe_service, studio_service};
use crate::utils::header_extractor::extract_language;
use actix_web::web::Data;
use actix_web::web::Query;
use actix_web::{HttpRequest, HttpResponse, web};
use std::ops::Deref;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/studio")
            .route("/stats", web::get().to(stats))
            .route("/recent-activity", web::get().to(recent_activity))
            .route("/by-filter", web::get().to(by_filter)),
    );
}

pub async fn stats(state: Data<AppState>, auth: AuthenticatedUser) -> Result<HttpResponse, Error> {
    let stats: DashboardStats = studio_service::get_stats(&state.db, auth.user.id).await?;

    Ok(HttpResponse::Ok().json(stats))
}

pub async fn recent_activity(
    state: Data<AppState>,
    auth: AuthenticatedUser,
    req: HttpRequest,
    query: Query<LastRecipesQuery>,
) -> Result<HttpResponse, Error> {
    let lang_code = extract_language(&req);

    let limit = query.nb.unwrap_or(4).min(20);
    let include_translations = query.include_translations.unwrap_or(false);

    let recipes = recipe_service::get_recent(
        &state.db,
        lang_code.deref(),
        limit,
        include_translations,
        auth.user.id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(recipes))
}
pub async fn by_filter(
    state: Data<AppState>,
    auth: AuthenticatedUser,
    req: HttpRequest,
    query: Query<GetAllRecipesByPageQuery>,
) -> Result<HttpResponse, Error> {
    let lang_code = extract_language(&req);

    let result = recipe_service::get_by_author_and_filter(&state.db, auth.user.id, query.into_inner(), &lang_code).await?;

    Ok(HttpResponse::Ok().json(result))
}
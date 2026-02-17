use std::ops::Deref;
use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use crate::app::state::AppState;
use crate::domain::user::AuthenticatedUser;
use crate::dto::ingredient_dto::{IngredientList, IngredientRecipeViewDto, IngredientViewDto};
use crate::dto::upload_dto::SingleImageForm;
use crate::errors::Error;
use crate::services::{ingredient_service, upload_service};
use crate::utils::header_extractor::extract_language;

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/ingredient")
            .route("/list", web::get().to(list))

    );
}

pub async fn list(
    state: web::Data<AppState>,
    query: web::Query<IngredientList>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let lang_code = extract_language(&req);
    let ingredients: Vec<IngredientViewDto> = ingredient_service::get_all(&state.db, query.search.clone(), query.limit, lang_code.deref()).await?;
    Ok(HttpResponse::Created().json(ingredients))
}
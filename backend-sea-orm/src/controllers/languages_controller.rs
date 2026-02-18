use actix_web::{web, HttpResponse};
use crate::app::state::AppState;
use crate::dto::language_dto::LanguageDto;
use crate::errors::Error;
use crate::services::{language_service};

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/language")
            .route("/list", web::get().to(list))

    );
}

pub async fn list(
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let ingredients: Vec<LanguageDto> = language_service::get_all(&state.db).await?;
    Ok(HttpResponse::Created().json(ingredients))
}
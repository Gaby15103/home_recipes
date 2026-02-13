use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::app::state::AppState;
use crate::domain::user::AuthenticatedUser;
use crate::dto::upload_dto::SingleImageForm;
use crate::errors::Error;
use crate::services::upload_service;

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/upload")
            .route("", web::post().to(upload_temp_image))

    );
}

pub async fn upload_temp_image(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
    MultipartForm(form): MultipartForm<SingleImageForm>,
) -> Result<HttpResponse, Error> {
    // 1. Service handles saving the file to a 'temp' directory
    let temp_id = upload_service::save_to_temp(&form.image).await?;

    // 2. Return the ID so the frontend can put it in the Recipe JSON later
    Ok(HttpResponse::Created().json(json!({ "temp_id": temp_id })))
}
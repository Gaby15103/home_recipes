use crate::dto::upload_dto::SingleImageForm;
use crate::errors::Error;
use crate::services::upload_service;
use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/upload")
            .route("", web::post().to(upload_temp_image))

    );
}

pub async fn upload_temp_image(
    MultipartForm(form): MultipartForm<SingleImageForm>,
) -> Result<HttpResponse, Error> {
    let temp_id = upload_service::save_to_temp(&form.image).await?;
    
    Ok(HttpResponse::Created().json(json!({ "temp_id": temp_id })))
}
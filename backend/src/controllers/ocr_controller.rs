use actix_web::{web, HttpRequest, HttpResponse};
use crate::errors::Error;
use reqwest::Client;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ocr")
            .route("/recipe", web::post().to(create_recipe))
    );
}
pub async fn create_recipe(
    image_data: web::Bytes
) ->Result<HttpResponse, Error>{
    let client = Client::new();
    let tesseract_url = "http://tesseract:8080/tesseract";

    let response = client
        .post(tesseract_url)
        .body(image_data)
        .send()
        .await
        .map_err(Error::from)?;

    let raw_text = response.text().await.map_err(Error::from)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "text": raw_text })))
}
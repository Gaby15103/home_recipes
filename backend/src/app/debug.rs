use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Error};
use futures_util::stream::StreamExt;
use serde_json::from_slice;
use std::str;
use crate::dto::{CreateRecipeInput, StepImageMeta};

pub async fn debug_multipart_parsed(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut recipe_input: Option<CreateRecipeInput> = None;
    let mut step_images_meta: Option<Vec<StepImageMeta>> = None;
    let mut main_image: Option<web::Bytes> = None;
    let mut step_images: Vec<web::Bytes> = vec![];

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| actix_web::error::ErrorBadRequest(e))?;

        let name = field.name().unwrap_or("<unknown>").to_string();
        let content_type = field.content_type().map(|ct| ct.to_string());

        let mut bytes = web::BytesMut::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk.map_err(|e| actix_web::error::ErrorBadRequest(e))?;
            bytes.extend_from_slice(&chunk);
        }

        match name.as_str() {
            "recipe" => {
                let json_str = str::from_utf8(&bytes)
                    .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

                // Deserialize {"recipe": {...}}
                let wrapper: serde_json::Value =
                    serde_json::from_str(json_str).map_err(|e| actix_web::error::ErrorBadRequest(e))?;
                let inner = wrapper
                    .get("recipe")
                    .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing 'recipe' key"))?;
                recipe_input = Some(
                    from_slice(inner.to_string().as_bytes())
                        .map_err(|e| actix_web::error::ErrorBadRequest(e))?,
                );
            }
            "step_images_meta" => {
                let json_str = str::from_utf8(&bytes)
                    .map_err(|e| actix_web::error::ErrorBadRequest(e))?;
                step_images_meta = Some(
                    from_slice(json_str.as_bytes())
                        .map_err(|e| actix_web::error::ErrorBadRequest(e))?,
                );
            }
            "main_image" => {
                main_image = Some(bytes.freeze());
            }
            "step_images" => {
                step_images.push(bytes.freeze());
            }
            other => {
                println!("Unknown field: {}", other);
            }
        }
    }

    if recipe_input.is_none() || main_image.is_none() || step_images_meta.is_none() {
        return Ok(HttpResponse::BadRequest().body("Missing required multipart fields"));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "recipe": recipe_input,
        "step_images_meta": step_images_meta,
        "main_image_bytes": main_image.unwrap().len(),
        "step_images_count": step_images.len()
    })))
}

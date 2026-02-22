use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};
use crate::errors::Error;
use crate::domain::user::{AuthenticatedUser, Role};
use crate::dto::upload_dto::SingleImageForm;
use crate::services::ocr_service;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ocr")
            .app_data(web::PayloadConfig::new(10 * 1024 * 1024))
            .route("/recipe", web::post().to(create_recipe))
    );
}
pub async fn create_recipe(
    auth: AuthenticatedUser,
    MultipartForm(form): MultipartForm<SingleImageForm>,
) ->Result<HttpResponse, Error>{
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;
    let recipe = ocr_service::recipe_from_file(form.image).await?;
    
    Ok(HttpResponse::Ok().json(recipe))
}
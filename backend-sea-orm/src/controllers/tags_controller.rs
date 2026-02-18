use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::Json;
use crate::app::state::AppState;
use crate::domain::user::{AuthenticatedUser, Role};
use crate::dto::tag_dto::{InputTag, TagDto};
use crate::errors::Error;
use crate::services::tag_service;

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/tags")
            .route("", web::get().to(list))
            .route("", web::post().to(create))
            .route("/{id}", web::put().to(update))
    );
}
pub async fn list(
    state: web::Data<AppState>,
    req: HttpRequest,
)-> Result<HttpResponse, Error>{
    let tags = tag_service::get_all(&state.db).await?;
    Ok(HttpResponse::Ok().json(tags))
}
pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    auth: AuthenticatedUser,
    input: Json<InputTag>
)-> Result<HttpResponse, Error>{
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;;
    let new_tag = input.into_inner();

    let result = tag_service::create(&state.db, new_tag).await?;
    Ok(HttpResponse::Ok().json(result))
}
pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    auth: AuthenticatedUser,
    input: Json<TagDto>
)-> Result<HttpResponse, Error>{
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;;
    let new_tag = input.into_inner();

    let result = tag_service::update(&state.db, new_tag).await?;
    Ok(HttpResponse::Ok().json(result))
}
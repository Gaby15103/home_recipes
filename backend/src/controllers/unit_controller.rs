use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::Json;
use uuid::Uuid;
use crate::app::state::AppState;
use crate::domain::user::{AuthenticatedUser, Role};
use crate::dto::unit_dto::{UnitDto, UnitInputDto};
use crate::errors::Error;
use crate::services::unit_service;

pub fn configure(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/units")
            .route("", web::get().to(list))
            .route("", web::post().to(create))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::get().to(get))
        
    );
}
pub async fn list(
    state: web::Data<AppState>,
)-> Result<HttpResponse, Error>{
    let tags = unit_service::get_all(&state.db).await?;
    Ok(HttpResponse::Ok().json(tags))
}
pub async fn create(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
    input: Json<UnitInputDto>
)-> Result<HttpResponse, Error>{
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;;
    let new_unit = input.into_inner();

    let result = unit_service::create(&state.db, new_unit).await?;
    Ok(HttpResponse::Ok().json(result))
}
pub async fn update(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
    input: Json<UnitDto>
)-> Result<HttpResponse, Error>{
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;;
    let updated_unit = input.into_inner();

    let result = unit_service::update(&state.db, updated_unit).await?;
    Ok(HttpResponse::Ok().json(result))
}
pub async fn get(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
)-> Result<HttpResponse, Error>{
    auth.require_roles(&[Role::Admin,Role::Moderator,Role::Superuser])?;;
    let unit_id = path.into_inner();
    let result = unit_service::get(&state.db, unit_id).await?;
    Ok(HttpResponse::Ok().json(result))
}
use crate::app::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use sea_orm::{ConnectionTrait, Statement};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
    pub version: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

pub async fn health_check(state: web::Data<AppState>) -> impl Responder {
    // Using your .ping() logic
    let is_alive = state.db.ping().await.is_ok();

    let db_status = if is_alive { "up" } else { "down" };
    let overall_status = if is_alive { "healthy" } else { "unhealthy" };

    HttpResponse::Ok().json(HealthResponse {
        status: overall_status.to_string(),
        database: db_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
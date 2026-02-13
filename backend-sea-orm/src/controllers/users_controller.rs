use actix_web::{web, HttpResponse};
use actix_web::web::Data;
use serde_json::json;
use uuid::Uuid;
use crate::app::state::AppState;
use crate::controllers::auth_controller::{confirm_email, forgot_password, login, logout, refresh, register, reset_password};
use crate::domain::user::AuthenticatedUser;
use crate::dto::user_dto::{UpdatePasswordDto, UpdateUserDto, UserResponseDto};
use crate::errors::Error;
use crate::services::user_service;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/me", web::get().to(get_me))
            .route("/profile", web::put().to(update_profile))
            .route("/password", web::put().to(change_password))
            .route("/sessions", web::get().to(get_sessions)),
    );
}

pub async fn get_me(
    auth: AuthenticatedUser,
) -> Result<HttpResponse, crate::errors::Error> {
    Ok(HttpResponse::Ok().json(auth.user))
}

pub async fn get_sessions(
    auth: AuthenticatedUser,
    state: web::Data<AppState>,
) -> Result<HttpResponse, crate::errors::Error> {
    let sessions = user_service::get_active_sessions(&state.db, auth.user.id, auth.active_session.id).await?;
    Ok(HttpResponse::Ok().json(sessions))
}

pub async fn terminate_session(
    state: Data<AppState>,
    auth: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let target_session_id = path.into_inner();

    // The service should verify the session belongs to the auth.user.id 
    // before deleting it (Security check!)
    user_service::revoke_session(&state.db, auth.user.id, target_session_id).await?;

    Ok(HttpResponse::Ok().json(json!({ "message": "Session terminated" })))
}

pub async fn change_password(
    auth: AuthenticatedUser,
    state: web::Data<AppState>,
    form: web::Json<UpdatePasswordDto>,
) -> Result<HttpResponse, crate::errors::Error> {
    // Call the specific password service
    user_service::change_password(&state.db, auth.user.id, form.into_inner()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Password updated successfully"
    })))
}

pub async fn update_profile(
    auth: AuthenticatedUser,
    state: web::Data<AppState>,
    form: web::Json<UpdateUserDto>,
) -> Result<HttpResponse, crate::errors::Error> {
    // Logic to update user names, avatar, etc.
    let updated_user = user_service::update_user(&state.db, auth.user.id, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(UserResponseDto::from((updated_user, auth.user.roles))))
}
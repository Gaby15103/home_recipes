use crate::app::state::AppState;
use crate::domain::user::AuthenticatedUser;
use crate::dto::user_dto::{UpdatePasswordDto, ProfileDto, UserResponseDto};
use crate::errors::Error;
use crate::services::user_service;
use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/me", web::get().to(get_me))
            .route("/{id}", web::get().to(get_by_id))
            .route("/profile/{id}", web::put().to(update_profile))
            .route("/password", web::put().to(change_password))
            .route("/sessions", web::get().to(get_sessions)),
    );
}

pub async fn get_me(
    auth: AuthenticatedUser,
) -> Result<HttpResponse, crate::errors::Error> {
    Ok(HttpResponse::Ok().json(auth.user))
}

pub async fn get_by_id(
    state: Data<AppState>,
    path: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    let user = user_service::get_by_id(&state.db, user_id).await?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn get_sessions(
    auth: AuthenticatedUser,
    state: Data<AppState>,
) -> Result<HttpResponse, crate::errors::Error> {
    let sessions = user_service::get_active_sessions(&state.db, auth.user.id, auth.active_session.id).await?;
    Ok(HttpResponse::Ok().json(sessions))
}

pub async fn terminate_session(
    state: Data<AppState>,
    auth: AuthenticatedUser,
    path: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let target_session_id = path.into_inner();

    // The service should verify the session belongs to the auth.user.id 
    // before deleting it (Security check!)
    user_service::revoke_session(&state.db, auth.user.id, target_session_id).await?;

    Ok(HttpResponse::Ok().json(json!({ "message": "Session terminated" })))
}

pub async fn change_password(
    auth: AuthenticatedUser,
    state: Data<AppState>,
    form: Json<UpdatePasswordDto>,
) -> Result<HttpResponse, crate::errors::Error> {
    // Call the specific password service
    user_service::change_password(&state.db, auth.user.id, form.into_inner()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Password updated successfully"
    })))
}

pub async fn update_profile(
    auth: AuthenticatedUser,
    state: Data<AppState>,
    path: Path<Uuid>,
    form: Json<ProfileDto>,
) -> Result<HttpResponse, crate::errors::Error> {
    let target_id = path.into_inner();

    let is_admin = auth.user.roles.iter().any(|r| r.name == "Admin" || r.name == "SuperAdmin");

    if auth.user.id != target_id && !is_admin {
        return Err(Error::Forbidden(json!({
            "message": "Access Denied: You can only update your own profile or require admin privileges."
        })));
    }

    let _existing_user = user_service::get_by_id(&state.db, target_id).await?;

    let profile = form.into_inner();
    let updated_user = user_service::update_user(&state.db, target_id, profile).await?;

    Ok(HttpResponse::Ok().json(UserResponseDto::from((updated_user, auth.user.roles))))
}
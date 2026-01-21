use super::AppState;
use crate::db::roles::fetch_roles_for_user;
use crate::dto::{ConfirmEmail, ConfirmEmailQuery, LoginResponse, LoginUser, RegisterResponse, RegisterUser, UpdateUser, UpdateUserOuter, UserResponse};
use crate::models::{Role, User};
use crate::prelude::*;
use crate::utils::{
    auth::{Auth, authenticate},
    jwt::CanGenerateJwt,
};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, HttpResponse, ResponseError, web::Data, web::Json, web};
use std::convert::From;
use uuid::Uuid;
use validator::Validate;
use crate::utils::email_service::send_email_confirmation;

#[derive(Debug, Deserialize)]
pub struct In<U> {
    user: U,
}

pub async fn register(
    form: Json<In<RegisterUser>>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let register_user = form.into_inner().user;

    register_user.validate()?;

    state
        .db
        .send(register_user)
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::Ok().finish())
}

pub async fn confirm_email_api(
    query: web::Query<ConfirmEmailQuery>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let token = query.token.clone();

    let res = state.db.send(ConfirmEmail { token }).await;

    match res {
        Ok(Ok(_)) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Email confirmed successfully! Please log in."
        }))),
        Ok(Err(err)) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": err.to_string()
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "message": "Internal server error"
        }))),
    }
}

pub async fn login(
    (form, state): (Json<In<LoginUser>>, Data<AppState>),
) -> Result<HttpResponse, Error> {
    let login_user = form.into_inner().user;

    // Validate input
    login_user.validate()?;

    let res = state
        .db
        .send(login_user)
        .await
        .map_err(|_| Error::InternalServerError)??;

    let mut response = HttpResponse::Ok();

    if !res.two_factor_required {
        let cookie = Cookie::build("session_id", res.session_id.to_string())
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .secure(false)
            .finish();
        response.cookie(cookie);
    }

    Ok(response.json(
        LoginResponse{
            two_factor_required: res.two_factor_required,
            two_factor_token: res.two_factor_token,
            user: Option::from(res.user)
        }
    ))
}
pub async fn get_current(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    Ok(HttpResponse::Ok().json(UserResponse::from_auth(auth)))
}

pub struct DeleteSession {
    pub session_id: Uuid,
}
pub async fn logout(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    if let Some(cookie) = req.cookie("session_id") {
        let session_id = cookie.value().parse::<uuid::Uuid>().ok();

        if let Some(id) = session_id {
            let _ = state.db.send(DeleteSession { session_id: id }).await;
        }
    }

    Ok(HttpResponse::Ok()
        .cookie(
            Cookie::build("session_id", "")
                .path("/")
                .http_only(true)
                .secure(false) // change to true if using HTTPS
                .max_age(time::Duration::seconds(0))
                .finish(),
        )
        .finish())
}

pub async fn update(
    state: Data<AppState>,
    (form, req): (Json<In<UpdateUser>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let update_user = form.into_inner().user;

    update_user.validate()?;

    let auth = authenticate(&state, &req).await?;

    let db = state.db.clone();

    let res = db
        .send(UpdateUserOuter { auth, update_user })
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}

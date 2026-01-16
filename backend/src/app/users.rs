use actix_web::{HttpRequest, HttpResponse, ResponseError, web::Data, web::Json};
use std::convert::From;
use actix_web::cookie::{Cookie, SameSite};
use uuid::Uuid;
use validator::Validate;
use crate::db::roles::fetch_roles_for_user;
use crate::dto::{LoginUser, RegisterUser, UpdateUser, UpdateUserOuter, UserResponse};
use super::AppState;
use crate::models::{Role, User};
use crate::prelude::*;
use crate::utils::{
    auth::{Auth, authenticate},
    jwt::CanGenerateJwt,
};

#[derive(Debug, Deserialize)]
pub struct In<U> {
    user: U,
}

pub async fn register(
    form: Json<In<RegisterUser>>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let register_user = form.into_inner().user;

    // Validate input
    register_user.validate()?;

    // Send to DbExecutor actor
    let res = state
        .db
        .send(register_user)
        .await
        .map_err(|_| Error::InternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
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

    let cookie = Cookie::build("session_id", res.session_id.to_string())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(false)
        .finish();

    Ok(
        HttpResponse::Ok()
            .cookie(cookie)
            .json(res.user),
    )
}
pub async fn get_current(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    Ok(
        HttpResponse::Ok()
            .json(UserResponse::from_auth(auth))
    )
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
        .finish()
    )
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

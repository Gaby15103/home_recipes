use actix_web::{HttpRequest, HttpResponse, ResponseError, web::Data, web::Json};
use std::convert::From;
use actix_web::cookie::{Cookie, SameSite};
use validator::Validate;
use crate::db::roles::fetch_roles_for_user;
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

// Client Messages ↓

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(length(min = 1, max = 20))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 72))]
    pub password: String,

    #[validate(length(min = 1, max = 20))]
    pub first_name: String,

    #[validate(length(min = 1, max = 20))]
    pub last_name: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email: String,
    #[validate(length(
        min = "8",
        max = "72",
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(length(min = 1, max = 20))]
    pub username: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub first_name: String,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub last_name: String,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub avatar_url: Option<String>,

    #[validate(custom(function = "validate_preferences"))]
    pub preferences: serde_json::Value,
}

fn validate_preferences(value: &serde_json::Value) -> Result<(), validator::ValidationError> {
    if value.is_null() {
        return Err(validator::ValidationError::new(
            "preferences cannot be null",
        ));
    }
    Ok(())
}

#[derive(Debug)]
pub struct UpdateUserOuter {
    pub auth: Auth,
    pub update_user: UpdateUser,
}

// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: UserResponseInner,
}

#[derive(Debug, Serialize)]
pub struct UserResponseInner {
    pub email: String,
    pub token: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub preferences: serde_json::Value,
    pub roles: Vec<Role>
}

impl UserResponse {
    pub fn from_user_and_roles(
        user: User,
        roles: Vec<Role>,
    ) -> Self {
        UserResponse {
            user: UserResponseInner {
                token: user.generate_jwt().unwrap(),
                email: user.email,
                username: user.username,
                first_name: user.first_name,
                last_name: user.last_name,
                avatar_url: user.avatar_url,
                preferences: user.preferences,
                roles,
            },
        }
    }

    pub fn from_auth(
        auth: Auth,
    ) -> Self {
        UserResponse {
            user: UserResponseInner {
                token: auth.token,
                email: auth.user.email,
                username: auth.user.username,
                first_name: auth.user.first_name,
                last_name: auth.user.last_name,
                avatar_url: auth.user.avatar_url,
                preferences: auth.user.preferences,
                roles: auth.roles,
            },
        }
    }
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

    let token = res.user.token.clone();

    let cookie = Cookie::build("access_token", token)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(false)
        .finish();

    Ok(
        HttpResponse::Ok()
            .cookie(cookie)
            .json(res)
    )
}
pub async fn get_current(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    Ok(
        HttpResponse::Ok()
            .json(UserResponse::from_auth(auth))
    )
}
pub async fn logout(state: Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::Ok()
        .cookie(
        Cookie::build("access_token", "")
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(0))
        .finish()
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

use std::ops::Deref;
use actix_multipart::form::json::Json;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::Data;
use chrono::{Duration, Utc};
use rand::distr::Alphanumeric;
use rand::Rng;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Iden, Set};
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;
use crate::app::state::AppState;
use crate::domain::user::NewUser;
use crate::dto::auth_dto::{LoginRequestDto, RegisterRequestDto};
use crate::dto::preferences_dto::{UserPreferences};
use crate::dto::user_dto::{LoginResponseDto, UserResponseDto};
use entity::{roles, sessions, users};
use crate::errors::Error;
use crate::repositories::{role_repository, session_repository, user_repository};
use crate::utils::{HASHER, PWD_SCHEME_VERSION};
use crate::utils::email_service::send_email_confirmation;
use crate::utils::password_verification::{check_password, verify_password};

pub async fn register(
    db: &DatabaseConnection,
    dto: RegisterRequestDto
) -> Result<HttpResponse, Error> {

    if user_repository::email_exists(db, &dto.email).await? {
        return Err(Error::EmailAlreadyExists);
    }

    let new_user = NewUser::try_from(dto)?;

    let (inserted_user, email_verification_token) = user_repository::create(db,new_user).await?;

    send_email_confirmation(inserted_user, &email_verification_token)?;

    Ok(HttpResponse::Created().finish())
}
pub async fn validate_session(
    db: &DatabaseConnection,
    token: &str,
) -> Result<Option<(sessions::Model, users::Model, Vec<roles::Model>)>, Error> {
    let result = session_repository::find_valid_session_with_user(db, token).await?;

    if let Some((session, user)) = result {
        let session_to_return = session.clone();
        let _ = session_repository::update_last_active(db, session).await;

        let roles = role_repository::get_roles_for_user(db, user.id).await?;

        return Ok(Some((session_to_return, user, roles)));
    }

    Ok(None)
}

//return access_token and refresh_token
pub async fn login(
    db: &DatabaseConnection,
    login_request: LoginRequestDto,
    user_agent: Option<String>,
    ip_address: Option<String>,
) -> Result<LoginResponseDto, Error> {
    let user = user_repository::find_by_email(db, &login_request.email).await?;

    verify_password(&user.password_hash, &login_request.password)?;

    if check_password(&user.password_hash, &login_request.password)? {
        let new_hash = HASHER.hash(&user.password_hash)?;
        user_repository::reset_password(db, user.id, new_hash).await?;
    }

    if user.two_factor_secret.is_some() && user.two_factor_confirmed_at.is_some() {
        return handle_two_factor(db, &user).await;
    }

    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let expires_at = (chrono::Utc::now() + chrono::Duration::days(30)).into();

    let session = session_repository::create(db, user.id, token, expires_at, user_agent, ip_address).await?;

    let roles = role_repository::get_roles_for_user(db, user.id).await?;


    Ok(LoginResponseDto::from((user, session, roles)))
}

async fn handle_two_factor(
    db: &DatabaseConnection,
    user: &users::Model
) -> Result<LoginResponseDto, Error> {

    let token = Uuid::new_v4();
    let expires = Utc::now() + Duration::minutes(10);

    let mut active: users::ActiveModel = user.clone().into();

    active.two_factor_token = Set(Some(token));
    active.two_factor_token_expires_at = Set(Some(DateTimeWithTimeZone::from(expires)));

    active.update(db).await?;

    Ok(LoginResponseDto {
        two_factor_required: true,
        two_factor_token: Some(token),
        session_token: None,
        user: None,
    })
}
pub async fn logout(
    db: &DatabaseConnection,
) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().json({}))
}
pub async fn refresh(
    db: &DatabaseConnection,
) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().json({}))
}
pub async fn confirm_email(
    db: &DatabaseConnection,
) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().json({}))
}
pub async fn forgot_password(
    db: &DatabaseConnection,
) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().json({}))
}
pub async fn reset_password(
    db: &DatabaseConnection,
) -> Result<HttpResponse, Error> {

    Ok(HttpResponse::Ok().json({}))
}
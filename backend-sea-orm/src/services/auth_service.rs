use crate::app::state::AppState;
use crate::domain::user::NewUser;
use crate::dto::auth_dto::{LoginRequestDto, RegisterRequestDto, ResetPasswordDto};
use crate::dto::preferences_dto::UserPreferences;
use crate::dto::user_dto::{LoginResponseDto, UpdatePasswordDto, UserResponseDto};
use crate::errors::Error;
use crate::repositories::{role_repository, session_repository, user_repository};
use crate::utils::email_service::{send_email_confirmation, send_password_reset};
use crate::utils::password_verification::{check_password, verify_password};
use crate::utils::{HASHER, PWD_SCHEME_VERSION};
use actix_multipart::form::json::Json;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, web};
use chrono::{Duration, Utc};
use entity::{roles, sessions, users};
use rand::Rng;
use rand::distr::Alphanumeric;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Iden, Set};
use std::ops::Deref;
use uuid::Uuid;

pub async fn register(
    db: &DatabaseConnection,
    dto: RegisterRequestDto,
) -> Result<HttpResponse, Error> {
    if user_repository::email_exists(db, &dto.email).await? {
        return Err(Error::EmailAlreadyExists);
    }

    let new_user = NewUser::try_from(dto)?;

    let (inserted_user, email_verification_token) = user_repository::create(db, new_user).await?;

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

    let session =
        session_repository::create(db, user.id, token, expires_at, user_agent, ip_address).await?;

    let roles = role_repository::get_roles_for_user(db, user.id).await?;

    Ok(LoginResponseDto::from((user, session, roles)))
}

async fn handle_two_factor(
    db: &DatabaseConnection,
    user: &users::Model,
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
pub async fn logout(db: &DatabaseConnection, token: &str) -> Result<(), Error> {
    let result = session_repository::delete_session_by_token(db, token).await?;
    Ok(())
}
pub async fn refresh_session(db: &DatabaseConnection, session_id: Uuid) -> Result<String, Error> {
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let expires_at = (chrono::Utc::now() + chrono::Duration::days(30)).into();

    let result = session_repository::refresh_session(db, session_id, token, expires_at).await?;
    Ok(result)
}
pub async fn confirm_email(
    db: &DatabaseConnection,
    token: Uuid
) -> Result<(), Error> {
    user_repository::confirm_email(db, token)
        .await?;

    Ok(())
}
pub async fn request_password_reset(
    db: &DatabaseConnection,
    email: &str,
) -> Result<(), Error> {

    let user = user_repository::find_by_email(db, email).await?;

    let token = Uuid::new_v4();

    user_repository::create_reset_token(db,user.id, token).await?;

    send_password_reset(user,&token)?;

    Ok(())
}
pub async fn reset_password(
    db: &DatabaseConnection,
    data: ResetPasswordDto,
) -> Result<(), Error> {
    let reset_token = user_repository::find_reset_token_by_token(db, data.token)
        .await?
        .ok_or(Error::BadRequest(serde_json::json!({"error": "Invalid token"})))?;

    let new_hash = HASHER.hash(&data.new_password)?;

    user_repository::reset_password(db, reset_token.user_id, new_hash).await?;

    session_repository::delete_all_user_sessions(db, reset_token.user_id).await?;

    user_repository::delete_reset_token_by_id(db, reset_token.id).await?;

    Ok(())
}

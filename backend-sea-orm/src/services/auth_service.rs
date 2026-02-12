use crate::app::state::AppState;
use crate::domain::user::NewUser;
use crate::dto::auth_dto::{LoginRequestDto, QrCodeResponse, RegisterRequestDto, ResetPasswordDto, VerifyTwoFactorRequest};
use crate::dto::preferences_dto::UserPreferences;
use crate::dto::user_dto::{LoginResponseDto, TwoFactorStatusResponse, UpdatePasswordDto, UserResponseDto, VerifyTwoFactorResult};
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
use qrcode::QrCode;
use serde_json::json;
use uuid::Uuid;
use crate::utils::two_factor::{generate_new_secret, verify_totp};

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
pub async fn get_or_create_2fa_secret(db: &DatabaseConnection, user: &users::Model) -> Result<String, Error> {
    let new_secret = generate_new_secret();
    if user.two_factor_secret.is_none() {
        user_repository::update_2fa_secret_if_null(db, user.id, new_secret.clone()).await?;
    }
    Ok(new_secret)
}

pub async fn generate_qr_code(user: &users::Model) -> Result<QrCodeResponse, Error> {
    let secret = user.two_factor_secret.as_ref()
        .ok_or(Error::BadRequest(json!({"error": "Secret not set"})))?;

    let otp_auth_url = format!(
        "otpauth://totp/HomeRecipes:{}?secret={}&issuer=HomeRecipes",
        user.email, secret
    );

    let code = QrCode::new(otp_auth_url.clone()).map_err(|_| Error::InternalServerError)?;
    let svg = code.render::<char>().min_dimensions(200, 200).build();

    Ok(QrCodeResponse { svg, url: otp_auth_url })
}

pub async fn verify_2fa_login(
    db: &DatabaseConnection,
    payload: VerifyTwoFactorRequest,
    user_agent: Option<String>,
    ip_address: Option<String>,
) -> Result<VerifyTwoFactorResult, Error> {
    // 1. Repository: Find the user
    let user = user_repository::find_user_by_2fa_token(db, payload.token).await?;

    let secret = user.two_factor_secret.as_ref()
        .ok_or(Error::Unauthorized(serde_json::json!({"error": "2FA not enabled"})))?;

    // 2. Utils/Repository: Verify logic
    let is_valid = if let Some(code) = payload.code {
        verify_totp(secret, &code)?
    } else if let Some(recovery) = payload.recovery_code {
        user_repository::consume_recovery_code(db, user.id, &recovery).await?
    } else {
        false
    };

    if !is_valid {
        return Err(Error::Unauthorized(serde_json::json!({"error": "Invalid code"})));
    }

    // 3. Repository: Cleanup
    user_repository::clear_2fa_token(db, user.id).await?;

    // 4. Session/Role Repository: Complete the login
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let expires_at = (chrono::Utc::now() + chrono::Duration::days(30)).into();

    let session =
        session_repository::create(db, user.id, token, expires_at, user_agent, ip_address).await?;
    let roles = role_repository::get_roles_for_user(db, user.id).await?;

    Ok(VerifyTwoFactorResult {
        user: UserResponseDto::from((user, roles)),
        session_token: session.token,
    })
}

pub async fn get_recovery_codes(db: &DatabaseConnection, user: &users::Model) -> Result<serde_json::Value, Error> {
    if let Some(existing) = &user.two_factor_recovery_codes {
        return Ok(existing.clone());
    }

    let new_codes: Vec<String> = (0..8).map(|_| {
        rand::thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect()
    }).collect();

    let json_codes = serde_json::to_value(new_codes).map_err(|_| Error::InternalServerError)?;
    user_repository::update_recovery_codes(db, user.id, json_codes.clone()).await?;

    Ok(json_codes)
}
pub async fn get_2fa_status(user: &users::Model) -> Result<TwoFactorStatusResponse, Error> {
    let enabled = user.two_factor_secret.is_some();
    let requires_confirmation = enabled && user.two_factor_confirmed_at.is_none();

    Ok(TwoFactorStatusResponse {
        enabled,
        requires_confirmation,
    })
}

pub async fn enable_2fa(db: &DatabaseConnection, user_id: Uuid) -> Result<(), Error> {
    user_repository::set_2fa_status(db, user_id, true).await
}

pub async fn disable_2fa_complete(db: &DatabaseConnection, user_id: Uuid) -> Result<(), Error> {
    user_repository::disable_2fa(db, user_id).await
}
use actix_web::{HttpRequest, HttpResponse, web};
use base32::encode as base32_encode;
use qrcode::QrCode;
use rand::distr::Alphanumeric;
use rand::Rng;
use serde::Serialize;
use serde_json::Value;
use super::AppState;
use crate::dto::*;
use crate::prelude::*;
use crate::utils::auth::{Auth, authenticate};

pub async fn secret_key(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let auth: Auth = authenticate(&state, &req).await?;
    let user = &auth.user;

    let secret = match &user.two_factor_secret {
        Some(s) if !s.is_empty() => s.clone(),
        _ => {
            let secret_bytes: [u8; 20] = {
                let mut arr = [0u8; 20];
                rand::rng().fill(&mut arr);
                arr
            };

            let secret = base32_encode(base32::Alphabet::RFC4648 { padding: false }, &secret_bytes);

            // Persist in DB
            let db_result = state
                .db
                .send(UpdateUserTwoFactorSecret {
                    user_id: user.id,
                    secret: secret.clone(),
                })
                .await;

            db_result
                .map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!("Mailbox error: {}", e))
                })?
                .unwrap();

            secret
        }
    };

    Ok(HttpResponse::Ok().json(SecretKeyResponse { secret_key: secret }))
}

pub async fn qr_code(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let auth: Auth = authenticate(&state, &req).await?;
    let user = &auth.user;

    let secret = match &user.two_factor_secret {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return Ok(HttpResponse::BadRequest().body("Secret not set")),
    };

    let otp_auth_url = format!(
        "otpauth://totp/MyApp:{}?secret={}&issuer=MyApp",
        user.email, secret
    );

    let code = QrCode::new(otp_auth_url.clone()).unwrap();
    let svg = code.render::<char>().min_dimensions(200, 200).build();

    Ok(HttpResponse::Ok().json(QrCodeResponse {
        svg,
        url: otp_auth_url,
    }))
}

pub async fn recovery_codes(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let auth: Auth = authenticate(&state, &req).await?;
    let user = &auth.user;

    let codes: Value = match &user.two_factor_recovery_codes {
        Some(codes) => codes.clone(),
        None => {
            // Generate 8 new codes
            let new_codes: Vec<String> = (0..8)
                .map(|_| {
                    rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(8)
                        .map(char::from)
                        .collect()
                })
                .collect();

            let json_codes = serde_json::to_value(new_codes.clone())
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            // Persist new codes in DB
            let db_result = state
                .db
                .send(UpdateUserRecoveryCodes {
                    user_id: user.id,
                    codes: json_codes.clone(),
                })
                .await;

            db_result
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Mailbox error: {}", e)))?
                .map_err(|e| actix_web::error::ErrorInternalServerError(format!("DB error: {:?}", e)))?;

            json_codes
        }
    };

    Ok(HttpResponse::Ok().json(codes))
}

pub async fn enable(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let auth: Auth = authenticate(&state, &req).await?;

    let db_result = state
        .db
        .send(UpdateUserTwoFactorEnabled {
            user_id: auth.user.id,
            enabled: true,
        })
        .await;

    db_result
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Mailbox error: {}", e))
        })?
        .unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub async fn disable(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let auth: Auth = authenticate(&state, &req).await?;

    let db_result = state
        .db
        .send(UpdateUserTwoFactorDisable {
            user_id: auth.user.id,
        })
        .await;

    db_result
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Mailbox error: {}", e))
        })?
        .unwrap();

    Ok(HttpResponse::Ok().finish())
}

pub async fn status(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let auth: Auth = authenticate(&state, &req).await?;
    let user = &auth.user;

    // Determine if 2FA is enabled
    let enabled = user.two_factor_secret.is_some();

    let requires_confirmation = enabled && user.two_factor_confirmed_at.is_none();

    let status = TwoFactorStatusResponse {
        enabled,
        requires_confirmation,
    };

    Ok(HttpResponse::Ok().json(status))
}


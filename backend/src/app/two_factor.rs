use crate::validator::Validate;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::web::Json;
use qrcode::QrCode;
use rand::distr::Alphanumeric;
use rand::Rng;
use serde_json::Value;
use super::AppState;
use crate::dto::*;
use crate::prelude::*;
use crate::utils::auth::{Auth, authenticate};
use crate::utils::two_factor::generate_new_secret;

pub async fn secret_key(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    let user = &auth.user;

    let secret = user
        .two_factor_secret
        .clone()
        .unwrap_or_else(|| generate_new_secret());

    state
        .db
        .send(UpdateUserTwoFactorSecret {
            user_id: user.id,
            secret: secret.clone(),
        })
        .await??;

    Ok(HttpResponse::Ok().finish()) // no need to return the secret
}


pub async fn qr_code(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
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
) -> Result<HttpResponse, Error> {
    // Authenticate user
    let auth: Auth = authenticate(&state, &req).await?;
    let user = &auth.user;

    let codes: Value = if let Some(existing) = &user.two_factor_recovery_codes {
        existing.clone()
    } else {
        let new_codes: Vec<String> = (0..8)
            .map(|_| {
                rand::rng()
                    .sample_iter(&Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect()
            })
            .collect();

        let json_codes = serde_json::to_value(new_codes.clone())
            .map_err(|_| Error::InternalServerError)?;
        state
            .db
            .send(UpdateUserRecoveryCodes {
                user_id: user.id,
                codes: json_codes.clone(),
            })
            .await??;

        json_codes
    };

    Ok(HttpResponse::Ok().json(codes))
}


pub async fn enable(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let auth: Auth = authenticate(&state, &req).await?;

    state
        .db
        .send(UpdateUserTwoFactorEnabled {
            user_id: auth.user.id,
            enabled: true,
        })
        .await??;

    Ok(HttpResponse::Ok().finish())
}

pub async fn disable(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let auth: Auth = authenticate(&state, &req).await?;

    state
        .db
        .send(UpdateUserTwoFactorDisable {
            user_id: auth.user.id,
        })
        .await??;

    Ok(HttpResponse::Ok().finish())
}


pub async fn status(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
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


pub async fn verify(
    state: web::Data<AppState>,
    form: Json<VerifyTwoFactorRequest>,
) -> Result<HttpResponse, Error> {
    let payload = form.into_inner();
    payload.validate()?;

    let res = state
        .db
        .send(VerifyTwoFactor(payload))
        .await
        .map_err(|_| Error::InternalServerError)??;

    let cookie = Cookie::build("session_id", res.session_id.to_string())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(false)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(res.user))
}

use crate::app::state::AppState;
use crate::domain::user::AuthenticatedUser;
use crate::dto::auth_dto::{ConfirmEmailQuery, ForgotPasswordDto, LoginRequestDto, RegisterRequestDto, ResetPasswordDto, SecretKeyResponse, VerifyTwoFactorRequest, VerifyTwoFactorResponse};
use crate::dto::user_dto::{LoginResponseDto, UserResponseDto};
use crate::errors::Error;
use crate::services::auth_service;
use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::web::Json;
use actix_web::web::{Data, service};
use actix_web::{HttpRequest, HttpResponse, web};
use serde_json::json;
use validator::Validate;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            // Auth Routes
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/refresh", web::post().to(refresh))
            .route("/confirm_email", web::post().to(confirm_email))
            .route("/forgot_password", web::post().to(forgot_password))
            .route("/reset_password", web::post().to(reset_password))
            // Nested 2FA Scope: This results in /auth/two-factor/...
            .service(
                web::scope("/two-factor") // Changed to hyphen to match your example
                    .route("/qr-code", web::get().to(qr_code)) // Fixed typo "qr-cod"
                    .route("/secret-key", web::get().to(secret_key))
                    .route("/recovery-codes", web::get().to(recovery_codes))
                    .route("/enable", web::post().to(enable))
                    .route("/disable", web::post().to(disable))
                    .route("/status", web::get().to(status))
                    .route("/verify", web::post().to(verify)),
            ),
    );
}

pub async fn register(
    form: Json<RegisterRequestDto>,
    state: Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let register_user = form.into_inner();

    register_user.validate()?;

    auth_service::register(&state.db, register_user).await?;

    Ok(HttpResponse::Created().finish())
}
//return access_token and refresh_token
pub async fn login(
    state: Data<AppState>,
    req: HttpRequest,
    form: Json<LoginRequestDto>,
) -> Result<HttpResponse, Error> {
    let login = form.into_inner();

    login.validate()?;

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let login_response: LoginResponseDto =
        auth_service::login(&state.db, login, user_agent, ip_address).await?;

    let mut response = HttpResponse::Ok();

    if !login_response.two_factor_required && login_response.session_token.is_some() {
        let cookie = Cookie::build(
            "session_token",
            login_response.session_token.clone().unwrap(),
        )
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(false)
        .max_age(Duration::days(30))
        .finish();
        response.cookie(cookie);
    }

    Ok(response.json(login_response))
}
pub async fn logout(
    state: web::Data<AppState>,
    req: HttpRequest,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    auth_service::logout(&state.db, &auth.active_session.token).await?;

    let mut response = HttpResponse::Ok();

    let cookie = Cookie::build("session_token", "")
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(false) // Set to true in production with HTTPS
        .max_age(Duration::ZERO) // This tells the browser to delete it immediately
        .finish();

    response.cookie(cookie);

    Ok(response.json(serde_json::json!({ "message": "Successfully logged out" })))
}
pub async fn refresh(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    // 1. Call service to swap the old token for a new one
    // We pass the current session ID to target the specific row
    let new_token = auth_service::refresh_session(&state.db, auth.active_session.id).await?;

    // 2. Build the new cookie
    let cookie = Cookie::build("session_token", new_token)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(false) // Set to true in prod
        .max_age(Duration::days(30))
        .finish();

    // 3. Return response with the updated cookie
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({ "status": "session rotated" })))
}
pub async fn confirm_email(
    state: web::Data<AppState>,
    query: web::Query<ConfirmEmailQuery>,
) -> Result<HttpResponse, Error> {
    // 1. Service handles the business logic and error mapping
    // If this fails, the '?' will automatically return the error to Actix
    auth_service::confirm_email(&state.db, query.token.clone()).await?;

    // 2. If it succeeds, return the success JSON
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Email confirmed successfully! Please log in."
    })))
}
pub async fn forgot_password(
    state: web::Data<AppState>,
    form: Json<ForgotPasswordDto>,
) -> Result<HttpResponse, Error> {
    form.validate()?;
    auth_service::request_password_reset(&state.db, &form.email).await?;

    Ok(HttpResponse::Ok()
        .json(json!({ "message": "If that email exists, a reset link has been sent" })))
}
pub async fn reset_password(
    state: web::Data<AppState>,
    form: Json<ResetPasswordDto>, // Contains new_password and token from email
) -> Result<HttpResponse, Error> {
    form.validate()?;
    auth_service::reset_password(&state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(json!({ "message": "Password updated successfully" })))
}

pub async fn secret_key(
    state: Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let secret = auth_service::get_or_create_2fa_secret(&state.db, &auth.user).await?;
    Ok(HttpResponse::Ok().json(SecretKeyResponse { secret_key: secret }))
}

pub async fn qr_code(auth: AuthenticatedUser) -> Result<HttpResponse, Error> {
    let res = auth_service::generate_qr_code(&auth.user).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn verify(
    state: Data<AppState>,
    req: HttpRequest,
    form: Json<VerifyTwoFactorRequest>,
) -> Result<HttpResponse, Error> {
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());
    let res = auth_service::verify_2fa_login(&state.db, form.into_inner(), user_agent, ip_address)
        .await?;

    let cookie = Cookie::build("session_token", res.session_token)
        .path("/")
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::days(30))
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(VerifyTwoFactorResponse { user: res.user }))
}
pub async fn recovery_codes(
    state: Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    let codes = auth_service::get_recovery_codes(&state.db, &auth.user).await?;
    Ok(HttpResponse::Ok().json(codes))
}
pub async fn enable(state: Data<AppState>, auth: AuthenticatedUser) -> Result<HttpResponse, Error> {
    auth_service::enable_2fa(&state.db, auth.user.id).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn disable(
    state: Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, Error> {
    auth_service::disable_2fa_complete(&state.db, auth.user.id).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn status(auth: AuthenticatedUser) -> Result<HttpResponse, Error> {
    let res = auth_service::get_2fa_status(&auth.user).await?;
    Ok(HttpResponse::Ok().json(res))
}

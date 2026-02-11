use crate::app::state::AppState;
use crate::dto::auth_dto::{LoginRequestDto, RegisterRequestDto};
use crate::errors::Error;
use crate::services::auth_service;
use actix_web::web::Json;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::cookie::time::Duration;
use validator::Validate;
use crate::dto::user_dto::{LoginResponseDto, UserResponseDto};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/refresh", web::post().to(refresh))
            .route("/confirm_email", web::post().to(confirm_email))
            .route("/forgot_password", web::post().to(forgot_password))
            .route("/reset_password", web::post().to(reset_password)),
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

    let user_agent = req.headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let ip_address = req.connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let login_response: LoginResponseDto = auth_service::login(&state.db, login, user_agent, ip_address).await?;

    let mut response = HttpResponse::Ok();
    
    if !login_response.two_factor_required && login_response.session_token.is_some(){
        let cookie = Cookie::build("session_token", login_response.session_token.clone().unwrap())
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
pub async fn logout(state: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn refresh(state: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn confirm_email(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn forgot_password(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}
pub async fn reset_password(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json({}))
}

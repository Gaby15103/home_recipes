use actix_cors::Cors;
use actix_web::{FromRequest, HttpMessage, HttpRequest, dev::Payload, Error};
use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::CONTENT_TYPE;
use actix_web::middleware::Next;
use actix_web::web::Data;
use crate::app::state::AppState;
use crate::domain::user::AuthenticatedUser;
use crate::services::auth_service;

pub fn cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:5173")
        .allowed_methods(vec!["GET","POST","PUT","DELETE","OPTIONS"])
        .allowed_headers(vec![CONTENT_TYPE])
        .supports_credentials()
        .max_age(3600)
}
pub async fn auth_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<impl MessageBody>, Error>
where
    B: MessageBody + 'static,
{
    let state = req.app_data::<Data<AppState>>().expect("AppState missing");

    if let Some(token_cookie) = req.cookie("session_token") {
        let token = token_cookie.value();

        if let Ok(Some((session, user, roles))) = auth_service::validate_session(&state.db, token).await {
            req.extensions_mut().insert(AuthenticatedUser {
                user,
                roles,
                active_session: session,
            });
        }
    }

    // Just call next.call(req).await
    // No need to map_into_boxed_body() anymore because we are generic
    next.call(req).await
}
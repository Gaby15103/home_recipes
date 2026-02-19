use actix_cors::Cors;
use actix_web::{FromRequest, HttpMessage, HttpRequest, dev::Payload, Error};
use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::CONTENT_TYPE;
use actix_web::middleware::Next;
use actix_web::web::Data;
use futures_util::future::{ready, Ready};
use serde_json::json;
use uuid::Uuid;
use crate::app::state::AppState;
use crate::domain::user::{AuthenticatedUser, Role};
use crate::dto::user_dto::UserResponseDto;
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
                user: UserResponseDto::from((user,roles.clone())),
                active_session: session,
            });
        }
    }

    // Just call next.call(req).await
    // No need to map_into_boxed_body() anymore because we are generic
    next.call(req).await
}
impl FromRequest for AuthenticatedUser {
    type Error = crate::errors::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Look into the extensions bucket where the middleware injected the user
        match req.extensions().get::<AuthenticatedUser>() {
            Some(auth_user) => ready(Ok(auth_user.clone())),
            None => ready(Err(crate::errors::Error::Unauthorized(json!({
                "error": "Authentication required"
            })))),
        }
    }
}
impl AuthenticatedUser {
    pub fn require_roles(&self, allowed: &[Role]) -> Result<(), crate::errors::Error> {
        let has_role = self.user.roles.iter().any(|user_role| {
            // Convert each 'allowed' enum to a string to compare against user_role.name
            allowed.iter().any(|a| a.to_string() == user_role.name)
        });

        if has_role {
            Ok(())
        } else {
            let allowed_names: Vec<String> = allowed.iter().map(|r| r.to_string()).collect();

            Err(crate::errors::Error::Forbidden(serde_json::json!({
                "error": "Access denied",
                "message": format!("One of these roles required: {:?}", allowed_names)
            })))
        }
    }
    pub fn require_owner_or_roles(
        &self,
        owner_id: Uuid,
        allowed: &[Role]
    ) -> Result<(), crate::errors::Error> {
        // 1. Check if user is the owner
        if self.user.id == owner_id {
            return Ok(());
        }

        // 2. If not owner, check roles (reusing your existing logic)
        let has_role = self.user.roles.iter().any(|user_role| {
            allowed.iter().any(|a| a.to_string() == user_role.name)
        });

        if has_role {
            Ok(())
        } else {
            Err(crate::errors::Error::Forbidden(serde_json::json!({
            "error": "Access denied",
            "message": "You must be the owner or have an authorized role to perform this action."
        })))
        }
    }
}

use actix_web::{HttpRequest,web::Data};
use serde_json::from_value;
use uuid::Uuid;
use crate::app::AppState;
use crate::models::{Role, User, UserPreferences};
use crate::prelude::*;

const TOKEN_PREFIX: &str = "Bearer ";

#[derive(Debug)]
pub struct Auth {
    pub user: User,
    pub session_id: Uuid,
    pub roles: Vec<Role>,
    pub preferences: UserPreferences,
}

pub struct GetSessionAuth {
    pub session_id: Uuid,
}
#[derive(Debug,Clone)]
pub struct SessionAuth {
    pub session_id: Uuid,
    pub user: User,
    pub roles: Vec<Role>,
}

pub async fn authenticate(state: &Data<AppState>, req: &HttpRequest) -> Result<Auth, Error> {
    let cookie = req
        .cookie("session_id")
        .ok_or_else(|| Error::Unauthorized(serde_json::json!({
            "error": "No session"
        })));

    let session_id = Uuid::parse_str(cookie?.value())
        .map_err(|_| Error::Unauthorized(serde_json::json!({
            "error": "Invalid session"
        })));

    let auth_data = state.db.send(GetSessionAuth { session_id: session_id? }).await??;

    if auth_data.session_id == Uuid::nil() {
        return Err(Error::Unauthorized(serde_json::json!({
            "error": "Session not found"
        })));
    }

    Ok(Auth {
        user: auth_data.user.clone(),
        session_id: auth_data.session_id,
        roles: auth_data.roles,
        preferences: from_value(auth_data.user.preferences.clone()).unwrap_or_else(|_| UserPreferences::default())
    })
}

use actix::Message;
use actix_web::{HttpRequest, http::header::AUTHORIZATION, web::Data};
use actix_web::http::header::HeaderValue;

use crate::app::AppState;
use crate::models::{Role, User};
use crate::prelude::*;

const TOKEN_PREFIX: &str = "Bearer ";

#[derive(Debug)]
pub struct Auth {
    pub user: User,
    pub token: String,
    pub roles: Vec<Role>,
}

#[derive(Debug)]
pub struct GenerateAuth {
    pub token: String,
}

pub async fn authenticate(state: &Data<AppState>, req: &HttpRequest) -> Result<Auth, Error> {
    let db = state.db.clone();
    let cookie = req
        .cookie("access_token")
        .ok_or_else(|| Error::Unauthorized(json!({
            "error": "No auth cookie"
        })))?;

    let token = cookie.value().to_string();

    let result = state
        .db
        .send(GenerateAuth { token })
        .await
        .map_err(|_| Error::InternalServerError)?;

    result
}

fn preprocess_authz_token(token: Option<&HeaderValue>) -> Result<String> {
    let token = match token {
        Some(token) => token.to_str().unwrap(),
        None => {
            return Err(Error::Unauthorized(json!({
                "error": "No authorization was provided",
            })));
        }
    };

    if !token.starts_with(TOKEN_PREFIX) {
        return Err(Error::Unauthorized(json!({
            "error": "Invalid authorization method",
        })));
    }

    Ok(token.replacen(TOKEN_PREFIX, "", 1))
}

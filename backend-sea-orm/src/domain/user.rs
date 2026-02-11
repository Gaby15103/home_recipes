use actix_web::{FromRequest, HttpMessage, HttpRequest};
use actix_web::dev::Payload;
use futures_util::future::{ready, Ready};
use sea_orm::JsonValue;
use serde_json::json;
use entity::{roles, sessions, users};
use crate::dto::auth_dto::RegisterRequestDto;
use crate::dto::preferences_dto::UserPreferences;
use crate::errors::Error;
use crate::utils::HASHER;

pub struct NewUser {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub avatar_url: String,
    pub preferences: JsonValue,
}

impl TryFrom<RegisterRequestDto> for NewUser {
    type Error = Error;

    fn try_from(dto: RegisterRequestDto) -> Result<Self, Self::Error> {
        Ok(Self {
            email: dto.email,
            username: dto.username,
            first_name: dto.first_name,
            last_name: dto.last_name,
            password_hash: HASHER.hash(&dto.password)
                .map_err(|e| Error::InternalServerError)?,
            avatar_url: "/assets/users/default.png".to_string(),
            preferences: json!({"language":"fr","theme":"Dark"}),
        })
    }
}
#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user: users::Model,
    pub roles: Vec<roles::Model>,
    pub active_session: sessions::Model,
}
impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Look into the extensions bucket where the middleware injected the user
        match req.extensions().get::<AuthenticatedUser>() {
            Some(auth_user) => ready(Ok(auth_user.clone())),
            None => ready(Err(Error::Unauthorized(json!({
                "error": "Authentication required"
            })))),
        }
    }
}
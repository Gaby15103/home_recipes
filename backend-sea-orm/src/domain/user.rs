use std::fmt;
use sea_orm::JsonValue;
use serde_json::json;
use entity::{roles, sessions, users};
use crate::dto::auth_dto::RegisterRequestDto;
use crate::dto::user_dto::UserResponseDto;
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
    pub user: UserResponseDto,
    pub active_session: sessions::Model,
}
#[derive(Clone, Debug)]
pub struct RequireRole {
    pub allowed_roles: Vec<String>,
}
#[derive(Clone, Debug)]
pub struct RequireRoleMiddleware<S> {
    service: S,
    allowed_roles: Vec<String>,
}
pub enum Role {
    Admin,
    User,
    Moderator,
    Superuser,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Role::Admin => "ADMIN",
            Role::User => "USER",
            Role::Moderator => "MODERATOR",
            Role::Superuser => "SUPER_ADMIN",
        };
        write!(f, "{}", name)
    }
}

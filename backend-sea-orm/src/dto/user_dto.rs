use chrono::{DateTime, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;
use entity::{roles, sessions, users};
use entity::users::Model;
use crate::dto::preferences_dto::UserPreferences;
use crate::dto::role_dto::RoleResponseDto;

#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: String,
    pub preferences: UserPreferences,
    pub email_verified: bool,
    pub last_login: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub roles: Vec<RoleResponseDto>,
    pub two_factor_secret: Option<String>,
    pub two_factor_recovery_codes: Option<serde_json::Value>,
    pub two_factor_comfirmed_at: Option<DateTime<Utc>>,
    pub two_factor_token: Option<Uuid>,
    pub two_factor_confirmed_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponseDto {
    pub user: Option<UserResponseDto>,
    pub session_token: Option<String>,
    pub two_factor_required: bool,
    pub two_factor_token: Option<Uuid>,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VerifyTwoFactorResult {
    pub user: UserResponseDto,
    pub session_token: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TwoFactorStatusResponse {
    pub enabled: bool,
    pub requires_confirmation: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub preferences: Option<UserPreferences>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePasswordDto {
    pub old_password: String,
    pub new_password: String,
}

impl From<(users::Model, sessions::Model, Vec<roles::Model>)> for LoginResponseDto {
    fn from((user, session, roles): (users::Model, sessions::Model, Vec<roles::Model>)) -> Self {
        Self{
            user: Some(UserResponseDto::from((user.clone(), roles))),
            session_token: Option::from(session.token.clone()),
            two_factor_required: user.two_factor_token.is_some(),
            two_factor_token: user.two_factor_token,
        }
    }
}
impl From<(users::Model, Vec<roles::Model>)> for UserResponseDto {
    fn from((user, roles): (entity::users::Model, Vec<entity::roles::Model>)) -> Self {
        // Handle JSON conversion
        let preferences: UserPreferences = serde_json::from_value(user.preferences)
            .unwrap_or_default(); // Ensure UserPreferences implements Default

        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            // Fixed typo: was user.email, now user.first_name
            first_name: user.first_name,
            last_name: user.last_name,
            avatar_url: user.avatar_url,
            preferences,
            email_verified: user.email_verified,
            // Handle Option<DateTime> safely
            last_login: user.last_login_at
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|| Utc::now()), // Fallback if never logged in
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(user.created_at, Utc),
            updated_at: DateTime::<Utc>::from_naive_utc_and_offset(user.updated_at, Utc),
            roles: roles.into_iter().map(RoleResponseDto::from).collect(),
            two_factor_secret: user.two_factor_secret,
            two_factor_recovery_codes: user.two_factor_recovery_codes,
            two_factor_comfirmed_at: Option::from(DateTime::<Utc>::from_naive_utc_and_offset(user.two_factor_confirmed_at.expect("REASON"), Utc)),
            two_factor_token: user.two_factor_token,
            two_factor_confirmed_at: user.two_factor_token_expires_at
                .map(|dt| dt.with_timezone(&Utc)),
        }
    }
}
impl From<(users::Model, Vec<RoleResponseDto>)> for UserResponseDto {
    fn from((user, roles): (entity::users::Model, Vec<RoleResponseDto>)) -> Self {
        // Handle JSON conversion
        let preferences: UserPreferences = serde_json::from_value(user.preferences)
            .unwrap_or_default(); // Ensure UserPreferences implements Default

        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            // Fixed typo: was user.email, now user.first_name
            first_name: user.first_name,
            last_name: user.last_name,
            avatar_url: user.avatar_url,
            preferences,
            email_verified: user.email_verified,
            // Handle Option<DateTime> safely
            last_login: user.last_login_at
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|| Utc::now()), // Fallback if never logged in
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(user.created_at, Utc),
            updated_at: DateTime::<Utc>::from_naive_utc_and_offset(user.updated_at, Utc),
            roles: roles.into_iter().map(RoleResponseDto::from).collect(),
            two_factor_secret: user.two_factor_secret,
            two_factor_recovery_codes: user.two_factor_recovery_codes,
            two_factor_comfirmed_at: Option::from(DateTime::<Utc>::from_naive_utc_and_offset(user.two_factor_confirmed_at.expect("REASON"), Utc)),
            two_factor_token: user.two_factor_token,
            two_factor_confirmed_at: user.two_factor_token_expires_at
                .map(|dt| dt.with_timezone(&Utc)),
        }
    }
}
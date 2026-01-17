use crate::models::{Role, User};
use crate::utils::auth::Auth;
use uuid::Uuid;
use crate::dto::EmailVerificationTokenResponse;
use crate::schema::email_verification_tokens::dsl::email_verification_tokens;
// Client Messages ↓

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(length(min = 1, max = 20))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 72))]
    pub password: String,

    #[validate(length(min = 1, max = 20))]
    pub first_name: String,

    #[validate(length(min = 1, max = 20))]
    pub last_name: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email: String,
    #[validate(length(
        min = "8",
        max = "72",
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(length(min = 1, max = 20))]
    pub username: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub first_name: String,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub last_name: String,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub avatar_url: Option<String>,

    #[validate(custom(function = "validate_preferences"))]
    pub preferences: serde_json::Value,
}
fn validate_preferences(
    value: &serde_json::Value,
) -> crate::prelude::Result<(), validator::ValidationError> {
    if value.is_null() {
        return Err(validator::ValidationError::new(
            "preferences cannot be null",
        ));
    }
    Ok(())
}

#[derive(Debug)]
pub struct UpdateUserOuter {
    pub auth: Auth,
    pub update_user: UpdateUser,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub two_factor_required: bool,
    pub two_factor_token: Option<Uuid>,
    pub user: Option<UserResponse>,
}


// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: UserResponseInner,
}

pub struct RegisterResponse {
    pub user: UserResponse,
    pub email_verification_tokens: EmailVerificationTokenResponse
}

pub struct ConfirmEmail {
    pub token: String,
}

#[derive(Deserialize)]
pub struct ConfirmEmailQuery {
    pub token: String,
}
#[derive(Debug, Serialize)]
pub struct UserResponseOuter {
    pub user: Option<UserResponse>,
    pub session_id: Uuid,
    pub two_factor_required: bool,
    pub two_factor_token: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct UserResponseInner {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub preferences: serde_json::Value,
    pub emai_verified: bool,
    pub roles: Vec<Role>,
}

impl UserResponse {
    pub fn from_user_and_roles(user: &User, roles: Vec<Role>) -> Self {
        UserResponse {
            user: UserResponseInner {
                email: user.email.clone(),
                username: user.username.clone(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                avatar_url: user.avatar_url.clone(),
                preferences: user.preferences.clone(),
                emai_verified: user.email_verified.unwrap().clone(),
                roles,
            },
        }
    }

    pub fn from_auth(auth: Auth) -> Self {
        UserResponse {
            user: UserResponseInner {
                email: auth.user.email,
                username: auth.user.username,
                first_name: auth.user.first_name,
                last_name: auth.user.last_name,
                avatar_url: auth.user.avatar_url,
                preferences: auth.user.preferences,
                emai_verified: auth.user.email_verified.unwrap().clone(),
                roles: auth.roles,
            },
        }
    }
}

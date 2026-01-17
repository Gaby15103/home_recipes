use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::dto::{UserResponse, UserResponseInner};
use crate::models::{EmailVerificationToken, Role, User};
use crate::utils::auth::Auth;

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyEmailRequest {
    pub token: Uuid,
}

#[derive(Debug, Serialize)]
pub struct VerifyEmailResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmailVerificationTokenResponse {
    pub user_id: Uuid,
    pub token: Uuid,
    pub created_at: DateTime<Utc>,
}
impl EmailVerificationTokenResponse {
    pub fn from_email_verification_token(email_verification_token: EmailVerificationToken) -> Self {
        EmailVerificationTokenResponse {
            user_id: email_verification_token.user_id,
            token: email_verification_token.token,
            created_at: email_verification_token.created_at,
        }
    }
}
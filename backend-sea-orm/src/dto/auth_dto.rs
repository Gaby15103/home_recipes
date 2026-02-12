use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterRequestDto {
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
pub struct LoginRequestDto {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 72))]
    pub password: String,
}
#[derive(Debug, Validate, Deserialize)]
pub struct ForgotPasswordDto {
    #[validate(email)]
    pub email: String,
}
#[derive(Debug, Validate, Deserialize)]
pub struct ResetPasswordDto {
    pub token: Uuid,
    #[validate(length(min = 8, max = 72))]
    pub new_password: String,
}
#[derive(Debug, Validate, Deserialize)]
pub struct ConfirmEmailQuery {
    pub token: Uuid,
}
#[derive(Debug, Deserialize, Validate)]
pub struct VerifyTwoFactorRequest {
    pub token: Uuid,

    #[validate(length(min = 6, max = 6))]
    pub code: Option<String>,

    pub recovery_code: Option<String>,
}
#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct QrCodeResponse {
    pub svg: String,
    pub url: String,
}
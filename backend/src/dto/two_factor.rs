use crate::models::User;
use uuid::Uuid;
use crate::dto::UserResponse;

#[derive(Serialize)]
pub struct QrCodeResponse {
    pub svg: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct SecretKeyResponse {
    pub secret_key: String,
}

#[derive(Serialize)]
pub struct RecoveryCodesResponse {
    pub codes: serde_json::Value,
}

#[derive(serde::Serialize)]
pub struct TwoFactorStatusResponse {
    pub enabled: bool,
    pub requires_confirmation: bool,
}

pub struct UpdateUserTwoFactorSecret {
    pub user_id: Uuid,
    pub secret: String,
}

pub struct UpdateUserRecoveryCodes {
    pub user_id: Uuid,
    pub codes: serde_json::Value,
}

pub struct UpdateUserTwoFactorEnabled {
    pub user_id: Uuid,
    pub enabled: bool,
}

pub struct UpdateUserTwoFactorDisable {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyTwoFactorRequest {
    pub token: Uuid,

    #[validate(length(min = 6, max = 6))]
    pub code: Option<String>,

    pub recovery_code: Option<String>,
}

pub struct VerifyTwoFactor(pub VerifyTwoFactorRequest);

pub struct VerifyTwoFactorResult {
    pub user: UserResponse,
    pub session_id: Uuid,
}

impl User {
    pub fn is_two_factor_enabled(&self) -> bool {
        self.two_factor_secret.is_some() && self.two_factor_confirmed_at.is_some()
    }

    pub fn has_recovery_codes(&self) -> bool {
        match &self.two_factor_recovery_codes {
            Some(codes) => !codes.as_array().unwrap_or(&vec![]).is_empty(),
            None => false,
        }
    }
}

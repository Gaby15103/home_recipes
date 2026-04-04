use crate::errors::Error;
use base32::{Alphabet, encode};
use chrono::Utc;
use rand::random;
use serde_json::json;
use totp_lite::{Sha1, totp_custom};

pub fn generate_new_secret() -> String {
    let secret_bytes: [u8; 20] = random();
    encode(Alphabet::RFC4648 { padding: false }, &secret_bytes)
}

pub fn verify_totp(secret: &str, code: &str) -> Result<bool, Error> {
    // 1. Validate User Input (400 Bad Request)
    let code_num: u32 = code.parse().map_err(|_| {
        Error::BadRequest(json!({
            "error": "Invalid TOTP code format",
            "received": code
        }))
    })?;

    // 2. Decode Secret (Log as 500 if the DB secret is corrupted)
    let secret_bytes =
        base32::decode(Alphabet::RFC4648 { padding: false }, secret).ok_or_else(|| {
            Error::InternalServerError(json!({
                "message": "Failed to decode 2FA secret from database",
                "operation": "verify_totp",
                "stage": "base32_decode",
                "secret_length": secret.len()
            }))
        })?;

    let timestamp = Utc::now().timestamp() as u64;

    // 3. Generate TOTP
    let generated_code = totp_custom::<Sha1>(30, 6, &secret_bytes, timestamp);

    // 4. Final Parse Check
    let generated_code_num: u32 = generated_code.parse().map_err(|e| {
        Error::InternalServerError(json!({
            "message": "TOTP library returned unparseable code",
            "operation": "verify_totp",
            "error": format!("{}", e),
            "generated_output": generated_code
        }))
    })?;

    Ok(generated_code_num == code_num)
}

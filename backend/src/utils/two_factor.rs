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
    let code_num: u32 = code
        .parse()
        .map_err(|_| Error::BadRequest(json!({"error": "Invalid TOTP code format"})))?;

    let secret_bytes = base32::decode(Alphabet::RFC4648 { padding: false }, secret)
        .ok_or_else(|| Error::BadRequest(json!({"error": "Invalid 2FA secret"})))?;

    let timestamp = Utc::now().timestamp() as u64;
    let generated_code = totp_custom::<Sha1>(30, 6, &secret_bytes, timestamp);

    let generated_code_num: u32 = generated_code
        .parse()
        .map_err(|_| Error::InternalServerError)?;

    Ok(generated_code_num == code_num)
}

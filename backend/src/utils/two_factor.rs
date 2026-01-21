use base32::{encode, Alphabet};
use totp_lite::{totp_custom, Sha1};
use chrono::Utc;
use diesel::prelude::*;
use rand::random;
use serde_json::Value;
use uuid::Uuid;

use crate::prelude::*;
use crate::schema::users::dsl::*;
use crate::models::User;

pub fn generate_new_secret() -> String {
    let secret_bytes: [u8; 20] = random();
    encode(Alphabet::RFC4648 { padding: false }, &secret_bytes)
}

/// Verify a TOTP code against the user's secret
pub fn verify_totp(secret: &str, code: &str) -> Result<bool, Error> {
    let code_num: u32 = code
        .parse()
        .map_err(|_| Error::BadRequest(json!({"error": "Invalid TOTP code format"})))?;

    // Decode the base32 secret stored in DB
    let secret_bytes = base32::decode(Alphabet::RFC4648 { padding: false }, secret)
        .ok_or_else(|| Error::BadRequest(json!({"error": "Invalid 2FA secret"})))?;

    // Just pass the raw timestamp (seconds since epoch)
    let timestamp = Utc::now().timestamp() as u64;

    let generated_code = totp_custom::<Sha1>(30, 6, &secret_bytes, timestamp);

    let generated_code_num: u32 = generated_code
        .parse()
        .map_err(|_| Error::BadRequest(json!({"error": "Invalid generated TOTP code"})))?;

    Ok(generated_code_num == code_num)
}

/// Consume a recovery code if it exists
pub fn consume_recovery_code(
    conn: &mut PgConnection,
    user_id_val: Uuid,
    code: &str,
) -> Result<bool> {
    let user: User = users.find(user_id_val).first(conn)?;

    let mut codes: Vec<String> = match &user.two_factor_recovery_codes {
        Some(Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        _ => vec![],
    };

    if let Some(pos) = codes.iter().position(|c| c == code) {
        codes.remove(pos); // remove used code
        diesel::update(users.find(user_id_val))
            .set(two_factor_recovery_codes.eq(Some(serde_json::json!(codes))))
            .execute(conn)?;
        return Ok(true);
    }

    Ok(false)
}

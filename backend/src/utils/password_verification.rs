use crate::errors::Error;
use crate::utils::PWD_SCHEME_VERSION;
use libreauth::pass::HashBuilder;
use serde_json::json;

pub fn verify_password(hash: &str, password: &str) -> Result<(), Error> {

    let checker = HashBuilder::from_phc(hash.trim())
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to parse password hash from database",
            "operation": "verify_password",
            "error": e.to_string(),
            "hash_prefix": hash.chars().take(10).collect::<String>(), // Log prefix only for safety
            "stage": "phc_parsing"
        })))?;

    // 2. Standard 403 for wrong credentials (don't log these as 500s!)
    if !checker.is_valid(password) {
        return Err(Error::Forbidden(json!({
            "error": "wrong password."
        })));
    }

    Ok(())
}
pub fn check_password(hash: &str, password: &str) -> Result<bool, Error> {
    let checker = HashBuilder::from_phc(hash.trim())
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to parse hash for update check",
            "operation": "check_password",
            "error": e.to_string()
        })))?;

    Ok(checker.needs_update(Some(PWD_SCHEME_VERSION)))
}

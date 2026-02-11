use jsonwebtoken::signature::digest::Mac;
use libreauth::pass::HashBuilder;
use serde_json::json;
use crate::errors::Error;
use crate::utils::{HASHER, PWD_SCHEME_VERSION};

pub fn verify_password(hash: &str, password: &str) -> Result<(), Error> {

    let checker = HashBuilder::from_phc(hash.trim())
        .map_err(|_| Error::InternalServerError)?;

    if !checker.is_valid(password) {
        return Err(Error::Forbidden(json!({
            "error": "wrong password."
        })));
    }

    Ok(())
}
pub fn check_password(hash: &str, password: &str) -> Result<bool, Error> {
    let checker = HashBuilder::from_phc(hash.trim())?;
    Ok(checker.needs_update(Some(PWD_SCHEME_VERSION)))
}

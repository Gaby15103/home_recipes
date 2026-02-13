use actix_multipart::form::tempfile::TempFile;
use crate::errors::Error;

pub async fn save_to_temp(
    file: &TempFile
) -> Result<String, Error>{
    Ok("save_to_temp".to_string())
}
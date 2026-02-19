use std::fs;
use std::path::{Path, PathBuf};
use actix_multipart::form::tempfile::TempFile;
use uuid::Uuid;
use crate::errors::Error;

pub async fn save_to_temp(
    file: &TempFile
) -> Result<String, Error>{
    let image_dir = PathBuf::from("assets/temp");
    
    fs::create_dir_all(&image_dir)?;

    let extension = file
        .file_name
        .as_deref()
        .and_then(|name| Path::new(name).extension())
        .and_then(|ext| ext.to_str())
        .unwrap_or("png");

    let file_name = format!(
        "recipe_{}_{}.{}",
        Uuid::new_v4(),
        chrono::Utc::now().timestamp(),
        extension
    );
    
    let disk_path = image_dir.join(&file_name);

    fs::copy(file.file.path(), &disk_path)?;
    
    Ok(format!("/assets/temp/{}", file_name))
}
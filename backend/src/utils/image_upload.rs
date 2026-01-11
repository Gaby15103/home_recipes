use actix_multipart::form::tempfile::TempFile;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use std::fs;

pub fn upload_recipe_image(
    temp_file: TempFile,
) -> Result<String, diesel::result::Error> {

    let image_dir = PathBuf::from("assets/recipes");

    // Ensure directory exists
    fs::create_dir_all(&image_dir).map_err(|e| {
        log::error!("Failed to create recipe image directory: {}", e);
        diesel::result::Error::RollbackTransaction
    })?;

    // Extract extension
    let extension = temp_file
        .file_name
        .as_deref()
        .and_then(|name| Path::new(name).extension())
        .and_then(|ext| ext.to_str())
        .unwrap_or("png");

    // Generate unique filename
    let file_name = format!(
        "recipe_{}_{}.{}",
        Uuid::new_v4(),
        chrono::Utc::now().timestamp(),
        extension
    );

    let disk_path = image_dir.join(&file_name);

    // Copy file to disk
    fs::copy(temp_file.file.path(), &disk_path).map_err(|e| {
        log::error!("Failed to copy recipe image: {}", e);
        diesel::result::Error::RollbackTransaction
    })?;

    // Public URL
    Ok(format!("/assets/recipes/{}", file_name))
}

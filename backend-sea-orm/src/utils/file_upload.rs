use std::fs;
use std::path::{Path, PathBuf};
use std::io;

/// Moves a file from its current path (e.g., assets/temp/...)
/// to the permanent recipe directory.
pub fn move_file_to_recipes(src_path: &str, target_dir: &str) -> Result<String, io::Error> {
    // 1. STRIP LEADING SLASH
    let sanitized_src = src_path.strip_prefix("/").unwrap_or(src_path);
    let path = Path::new(sanitized_src);

    // 2. Extract the filename safely
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid filename"))?;

    // 3. Build destination
    let dest_path = PathBuf::from(target_dir).join(file_name);

    // 4. Debug check
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source file not found at: {}", sanitized_src)
        ));
    }

    // 5. Move the file
    fs::rename(sanitized_src, &dest_path)?;

    // 6. FIX: Return the NEW path, starting with / for the frontend
    // Assuming target_dir is "assets/recipes"
    Ok(format!("/{}", dest_path.to_string_lossy()))
}
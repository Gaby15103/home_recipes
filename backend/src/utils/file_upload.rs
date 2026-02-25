use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn move_file_to_recipes(src_path: &str, target_dir: &str) -> Result<String, io::Error> {
    let sanitized_src = src_path.strip_prefix("/").unwrap_or(src_path);
    let path = Path::new(sanitized_src);

    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid filename"))?;

    let dest_path = PathBuf::from(target_dir).join(file_name);

    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source file not found at: {}", sanitized_src)
        ));
    }

    fs::rename(sanitized_src, &dest_path)?;
    
    Ok(format!("/{}", dest_path.to_string_lossy()))
}
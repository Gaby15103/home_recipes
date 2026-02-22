use std::ops::Deref;
use actix_multipart::form::tempfile::TempFile;
use actix_web::web;
use tesseract_rs::TesseractAPI;
use crate::dto::recipe_dto::CreateRecipeInput;
use crate::errors::Error;
use crate::utils::ollama;

pub async fn recipe_from_file(
    image: TempFile
)->Result<CreateRecipeInput, Error>{
    // 1. Offload heavy CPU work to web::block
    let text = web::block(move || {
        // Initialize API
        let api = TesseractAPI::new();

        // Explicitly point to the tessdata folder itself
        let home = std::env::var("HOME").unwrap_or_default();
        let tessdata_path = format!("{}/.tesseract-rs/tessdata", home);

        println!("Initializing Tesseract with exact path: {}", tessdata_path);

        // Initialize
        api.init(&tessdata_path, "fra+eng").map_err(|e| {
            eprintln!("Tesseract Init Error: {:?}", e);
            "Failed to init Tesseract"
        })?;
        // Use .path() to get the actual location in /tmp
        let path = image.file.path();

        // Read the actual bytes from the disk into memory
        let file_bytes = std::fs::read(path)
            .map_err(|_| "Could not read temp file from disk")?;

        println!("Read {} bytes from disk. Attempting memory decode...", file_bytes.len());

        // Use load_from_memory - it's often better at sniffing headers than open()
        let img = image::load_from_memory(&file_bytes)
            .map_err(|e| {
                eprintln!("Detailed Image Error: {:?}", e);
                "Failed to decode image data"
            })?
            .to_rgb8();

        let (width, height) = img.dimensions();

        api.set_image(
            &img.into_raw(),
            width as i32,
            height as i32,
            3,
            (width * 3) as i32
        ).map_err(|_| "Failed to set image")?;

        api.get_utf8_text().map_err(|_| "OCR execution failed")
    })
        .await
        .map_err(|_| Error::InternalServerError)? // Threadpool error
        .map_err(|e| Error::BadRequest(serde_json::json!({ "error": e })))?;
    let recipe = ollama::process_ocr_to_dto(&text.deref()).await?;
    Ok(recipe)
}
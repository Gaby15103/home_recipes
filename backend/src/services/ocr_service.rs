use crate::dto::recipe_dto::CreateRecipeInput;
use crate::errors::Error;
use crate::repositories::unit_repository;
use crate::utils::llm_prompt;
use actix_multipart::form::tempfile::TempFile;
use actix_web::web;
use sea_orm::DatabaseConnection;
use std::ops::Deref;
use std::time::Instant;
use tesseract_rs::TesseractAPI;

pub async fn recipe_from_file(
    image: TempFile,
    db: &DatabaseConnection,
) -> Result<CreateRecipeInput, Error> {
    let units = unit_repository::get_all_admin(db).await?;
    let start = Instant::now();
    let text = web::block(move || {
        let api = TesseractAPI::new();

        let tessdata_path = "/usr/share/tessdata";

        println!("Initializing Tesseract with system path: {}", tessdata_path);

        api.init(tessdata_path, "fra+eng").map_err(|e| {
            eprintln!("Tesseract Init Error: {:?}", e);
            "Failed to init Tesseract"
        })?;
        let path = image.file.path();
        
        let file_bytes = std::fs::read(path).map_err(|_| "Could not read temp file from disk")?;

        println!(
            "Read {} bytes from disk. Attempting memory decode...",
            file_bytes.len()
        );
        
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
            (width * 3) as i32,
        )
        .map_err(|_| "Failed to set image")?;

        api.get_utf8_text().map_err(|_| "OCR execution failed")
    })
    .await
    .map_err(|_| Error::InternalServerError)? // Threadpool error
    .map_err(|e| Error::BadRequest(serde_json::json!({ "error": e })))?;

    let duration = start.elapsed();
    println!("OCR processing finished in {:.2}s", duration.as_secs_f64());

    let llm_start = Instant::now();

    let recipe = llm_prompt::process_ocr_to_dto(&text.deref(), units).await?;

    println!("LLM processing finished in {:.2}s", llm_start.elapsed().as_secs_f64());
    println!("Total recipe extraction took: {:.2}s", start.elapsed().as_secs_f64());

    Ok(recipe)
}

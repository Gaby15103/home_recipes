use crate::errors::Error;
use image::GenericImageView;
use std::path::Path;
use std::time::Instant;
use tesseract_rs::TesseractAPI;
use Error::InternalServerError;
pub fn scan_image(image_path: &Path) -> Result<String, Error> {
    let start = Instant::now();
    let api = TesseractAPI::new();

    let tessdata_path = "/usr/share/tessdata";
    api.init(tessdata_path, "eng+fra").map_err(|e| {
        InternalServerError
    })?;

    let img = image::open(image_path)
        .map_err(|_| Error::BadRequest(serde_json::json!({"error": "Failed to open image file"})))?
        .to_rgb8();

    let (width, height) = img.dimensions();

    api.set_image(
        &img.into_raw(),
        width as i32,
        height as i32,
        3,
        (width * 3) as i32,
    ).map_err(|_| InternalServerError)?;

    let text = api.get_utf8_text().map_err(|_| {
        InternalServerError
    })?;

    let duration = start.elapsed();
    println!("⏱️ [Scanner] OCR completed in: {:.2?}", duration);

    Ok(text)
}
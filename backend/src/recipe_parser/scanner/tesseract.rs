use crate::errors::Error;
use tesseract_rs::TesseractAPI;
use std::path::Path;

pub struct OcrLine {
    pub text: String,
    pub y_pos: i32,
    pub height: i32,
    pub confidence: f32,
}

pub fn scan_single_image(path: &Path, lang: &str) -> Result<(Vec<OcrLine>, String), Error> {
    let mut api = TesseractAPI::new(); // Usually needs to be mutable
    api.init("/usr/share/tessdata", lang)
        .map_err(|_| Error::InternalServerError)?;

    // 1. Properly handle the file read result
    let file_bytes = std::fs::read(path).map_err(|e| {
        eprintln!("❌ IO Error reading {:?}: {}", path, e);
        Error::InternalServerError
    })?;

    println!(
        "📸 Read {} bytes from disk. Attempting memory decode...",
        file_bytes.len()
    );

    // 2. Decode bytes to image
    let img = image::load_from_memory(&file_bytes)
        .map_err(|e| {
            eprintln!("❌ Detailed Image Error: {:?}", e);
            Error::BadRequest(serde_json::json!({"error": format!("Format error: {}", e)}))
        })?
        .to_rgb8();

    let (w, h) = img.dimensions();

    // 3. Pass raw buffer to Tesseract
    // img.into_raw() returns the Vec<u8> of RGB pixels
    api.set_image(&img.into_raw(), w as i32, h as i32, 3, (w * 3) as i32)
        .map_err(|_| Error::InternalServerError)?;

    let text = api.get_utf8_text().map_err(|_| Error::InternalServerError)?;

    let lines = text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| OcrLine {
            text: l.trim().to_string(),
            y_pos: 0,
            height: 0,
            confidence: 1.0,
        })
        .collect();

    Ok((lines,text))
}
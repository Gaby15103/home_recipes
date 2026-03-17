use crate::errors::Error;
use crate::recipe_parser::scanner::tesseract::scan_single_image;
use sqlx::SqlitePool;
use std::path::Path;
use image::GenericImageView;
use serde_derive::{Deserialize, Serialize};
use regex::Regex;
use crate::dto::upload_dto::RegionDto;

pub mod tesseract;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScannedDocument {
    pub raw_lines: Vec<String>,
    pub detected_lang: String,
    pub raw_text: String,
}

pub async fn process_batch(
    paths: Vec<&Path>,
    lang: &str,
    _pool: &SqlitePool,
) -> Result<ScannedDocument, Error> {
    let mut all_lines: Vec<String> = Vec::new();
    let mut raw_text = String::new();

    for path in paths {
        let (lines, text) = scan_single_image(path, lang)?;
        raw_text.push_str(&text);

        for line in lines {
            let cleaned = clean_ocr_typos(line.text);
            if cleaned.is_empty() { continue; }

            if all_lines.last().map_or(true, |last| last != &cleaned) {
                all_lines.push(cleaned);
            }
        }
    }

    Ok(ScannedDocument {
        raw_lines: all_lines,
        detected_lang: lang.to_string(),
        raw_text,
    })
}

pub fn scan_region(path: &std::path::Path, region: &RegionDto, lang: &str) -> Result<String, Error> {
    // 1. Use the logic that you know works to get the file
    let file_bytes = std::fs::read(path).map_err(|e| {
        log::error!("Failed to read image file at {:?}: {}", path, e);
        Error::InternalServerError
    })?;

    // 2. Load the image from memory (more reliable than image::open)
    let img = image::load_from_memory(&file_bytes).map_err(|e| {
        log::error!("Failed to decode image from memory: {}", e);
        Error::InternalServerError
    })?;

    let (img_w, img_h) = img.dimensions();

    // 3. Mathematical Clamping (to prevent crop panics)
    let x = region.x.min(img_w);
    let y = region.y.min(img_h);
    let w = region.w.min(img_w - x);
    let h = region.h.min(img_h - y);

    if w == 0 || h == 0 {
        return Ok("".to_string());
    }

    // 4. Crop and convert back to bytes for your preprocessor
    let cropped = img.crop_imm(x, y, w, h);
    let mut buf = std::io::Cursor::new(Vec::new());
    cropped.write_to(&mut buf, image::ImageFormat::Png).map_err(|_| Error::InternalServerError)?;

    // 5. Use your proven preprocessing and engine
    let clean_png_bytes = tesseract::preprocess_for_ocr(&buf.into_inner());

    // Using PSM 6 for specific regions is usually the sweet spot
    let (_, text) = tesseract::run_tesseract_engine(&clean_png_bytes, "eng+fra", "6")?;

    Ok(text.trim().to_string())
}

fn clean_ocr_typos(text: String) -> String {
    // 1. Normalize vulgar fractions immediately
    let mut t = text
        .replace('½', " 1/2")
        .replace('¼', " 1/4")
        .replace('¾', " 3/4")
        .replace("I/2", "1/2") // Common Tesseract error
        .replace("I/4", "1/4")
        .replace("mlnute", "minute")
        .replace("atab)", "c. à tab"); // Specific to your recipe's handwritten-style OCR

    // 2. Remove "Stray Vertical Bars" often caused by the lines in your photo
    let re_pipes = regex::Regex::new(r"[|¦!]").unwrap();
    t = re_pipes.replace_all(&t, "").to_string();

    // 3. Strip leading junk characters like "î" or "%"
    // that appear before the actual quantity "250 ml"
    let re_junk_prefix = regex::Regex::new(r"^[^a-zA-Z\d\s/¼½¾]{1,2}\s+").unwrap();
    t = re_junk_prefix.replace(&t, "").to_string();

    t.trim().to_string()
}
use crate::errors::Error;
use tesseract_rs::{TessPageIteratorLevel, TesseractAPI};
use std::path::Path;
use std::sync::Once;
use image::{DynamicImage, GenericImageView, Luma};
use magick_rust::{MagickWand, magick_wand_genesis, ColorspaceType};
use serde_json::json;

pub struct OcrLine {
    pub text: String,
    pub y_pos: i32,
    pub height: i32,
    pub confidence: f32,
}
static START: Once = Once::new();

pub fn scan_single_image(path: &Path, lang: &str) -> Result<(Vec<OcrLine>, String), Error> {
    let file_bytes = std::fs::read(path).map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to read image file",
        "operation": "scan_single_image",
        "file_path": path.to_string_lossy(),
        "language": lang,
        "error": e.to_string(),
        "stage": "file_read"
    })))?;
    let clean_png_bytes = preprocess_for_ocr(&file_bytes);

    // Pass 1: Try PSM 4 (Optimized for columns/recipes)
    let (mut lines, mut full_text) = run_tesseract_engine(&clean_png_bytes, lang, "4")?;

    // Validation: If it looks like garbage or merged text, try Pass 2
    if !is_ocr_result_valid(&full_text) {
        // Pass 2: Fallback to PSM 11 (Ignores layout logic, just finds text)
        if let Ok((fallback_lines, fallback_text)) = run_tesseract_engine(&clean_png_bytes, lang, "11") {
            lines = fallback_lines;
            full_text = fallback_text;
        }
    }

    Ok((lines, full_text))
}

pub fn scan_image_segment(img: DynamicImage, lang: &str) -> Result<String, Error> {
    // 1. Convert to RGB8 as Tesseract expects
    let (w, h) = img.dimensions();
    let raw_data = img.to_rgb8();

    // 2. Init API (Same as your run_tesseract_engine)
    let mut api = TesseractAPI::new();
    api.init("/usr/share/tessdata", lang).map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to initialize Tesseract API",
        "operation": "scan_image_segment",
        "language": lang,
        "image_width": w,
        "image_height": h,
        "error": e.to_string(),
        "stage": "tesseract_init"
    })))?;

    // For single regions, PSM 7 (Single line) or PSM 6 (Block) is often better
    api.set_variable("tessedit_pageseg_mode", "6").map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to set Tesseract PSM variable",
        "operation": "scan_image_segment",
        "psm": "6",
        "error": e.to_string(),
        "stage": "tesseract_config"
    })))?;

    api.set_image(&raw_data.into_raw(), w as i32, h as i32, 3, (w * 3) as i32)
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to set image data for Tesseract",
            "operation": "scan_image_segment",
            "image_width": w,
            "image_height": h,
            "error": e.to_string(),
            "stage": "image_setup"
        })))?;

    api.recognize().map_err(|e| Error::InternalServerError(json!({
        "message": "Tesseract OCR recognition failed",
        "operation": "scan_image_segment",
        "error": e.to_string(),
        "stage": "recognition"
    })))?;

    let text = api.get_utf8_text().map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to extract text from Tesseract results",
        "operation": "scan_image_segment",
        "error": e.to_string(),
        "stage": "text_extraction"
    })))?;

    Ok(text.trim().to_string())
}

pub(crate) fn run_tesseract_engine(img_bytes: &[u8], lang: &str, psm: &str) -> Result<(Vec<OcrLine>, String), Error> {
    let mut api = TesseractAPI::new();
    api.init("/usr/share/tessdata", lang).map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to initialize Tesseract API in run_tesseract_engine",
        "operation": "run_tesseract_engine",
        "language": lang,
        "psm": psm,
        "error": e.to_string(),
        "stage": "tesseract_init"
    })))?;

    api.set_variable("tessedit_pageseg_mode", psm).map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to set PSM variable",
        "operation": "run_tesseract_engine",
        "psm": psm,
        "error": e.to_string(),
        "stage": "set_variable"
    })))?;
    api.set_variable("preserve_interword_spaces", "1").map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to set preserve_interword_spaces variable",
        "operation": "run_tesseract_engine",
        "error": e.to_string(),
        "stage": "set_variable"
    })))?;

    let img = image::load_from_memory(img_bytes)
        .map_err(|e| Error::BadRequest(json!({
            "error": "Failed to decode image from memory",
            "details": e.to_string(),
            "size_bytes": img_bytes.len()
        })))?;

    let mut gray_img = img.grayscale().to_luma8();

    for pixel in gray_img.pixels_mut() {
        if pixel.0[0] < 140 {
            *pixel = Luma([0u8]);
        } else {
            *pixel = Luma([255u8]);
        }
    }

    let (w, h) = gray_img.dimensions();
    let final_rgb = DynamicImage::ImageLuma8(gray_img).to_rgb8();

    api.set_image(&final_rgb.into_raw(), w as i32, h as i32, 3, (w * 3) as i32)
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to set image for Tesseract",
            "operation": "run_tesseract_engine",
            "image_width": w,
            "image_height": h,
            "error": e.to_string(),
            "stage": "set_image"
        })))?;

    api.recognize().map_err(|e| Error::InternalServerError(json!({
        "message": "Tesseract recognition failed",
        "operation": "run_tesseract_engine",
        "psm": psm,
        "language": lang,
        "error": e.to_string(),
        "stage": "recognize"
    })))?;

    let full_text = api.get_utf8_text().map_err(|e| Error::InternalServerError(json!({
        "message": "Failed to extract text from Tesseract",
        "operation": "run_tesseract_engine",
        "error": e.to_string(),
        "stage": "get_utf8_text"
    })))?;

    let mut ocr_lines = Vec::new();

    if let Ok(it) = api.get_iterator() {
        loop {
            if let Ok(text) = it.get_utf8_text(TessPageIteratorLevel::RIL_TEXTLINE) {

                let trimmed = text.trim();

                if !trimmed.is_empty() {
                    let cleaned_text = trimmed
                        .replace("mi ", "ml ")
                        .replace("mt ", "ml ")
                        .replace("(1 1)", "(1 t)")
                        .replace("—", "")
                        .replace("_", "")
                        .replace("  ", " ")
                        .trim()
                        .to_string();

                    if let Ok((_, top, _, bottom)) = it.get_bounding_box(TessPageIteratorLevel::RIL_TEXTLINE) {
                        let conf = it.confidence(TessPageIteratorLevel::RIL_TEXTLINE).unwrap_or(0.0);

                        ocr_lines.push(OcrLine {
                            text: cleaned_text,
                            y_pos: top,
                            height: bottom - top,
                            confidence: conf / 100.0,
                        });
                    }
                }
            }

            match it.next(TessPageIteratorLevel::RIL_TEXTLINE) {
                Ok(true) => continue,
                _ => break,
            }
        }
    }

    Ok((ocr_lines, full_text))
}

pub fn preprocess_for_ocr(image_data: &[u8]) -> Vec<u8> {
    START.call_once(|| {
        magick_wand_genesis();
    });

    let mut wand = MagickWand::new();
    wand.read_image_blob(image_data).expect("Failed to load image");

    wand.set_image_colorspace(ColorspaceType::GRAY).expect("Failed to set colorspace");
    wand.deskew_image(40.0).ok();

    // Thresholding via FFI
    unsafe {
        use magick_rust::bindings::{MagickThresholdImage, MagickWand as RawWand};

        // We cast the MagickWand wrapper to a pointer of its internal type.
        // In magick-rust, the first field of MagickWand is the *mut MagickWand pointer.
        let raw_ptr: *mut RawWand = *( (&wand as *const MagickWand) as *const *mut RawWand );

        let threshold = 0.6 * 65535.0;
        MagickThresholdImage(raw_ptr, threshold);
    }

    wand.set_image_format("png").expect("Failed to set format");
    wand.write_image_blob("png").expect("Failed to create PNG blob")
}

fn is_ocr_result_valid(text: &str) -> bool {
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.is_empty() { return false; }

    // Check for "Horizontal Smearing"
    // If a line contains more than two 'units' (ml, g, lb),
    // it likely merged two columns incorrectly.
    let merged_count = lines.iter()
        .filter(|l| {
            let units = ["ml", "g", "t.", "c."];
            units.iter().filter(|&&u| l.to_lowercase().contains(u)).count() > 1
        })
        .count();

    // If more than 20% of lines look merged, this PSM is a failure
    if (merged_count as f32 / lines.len() as f32) > 0.2 {
        return false;
    }

    true
}
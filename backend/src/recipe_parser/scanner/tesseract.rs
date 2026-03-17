use crate::errors::Error;
use tesseract_rs::TesseractAPI;
use std::path::Path;
use std::sync::Once;
use image::{DynamicImage, GenericImageView};
use magick_rust::{MagickWand, magick_wand_genesis, ColorspaceType};

pub struct OcrLine {
    pub text: String,
    pub y_pos: i32,
    pub height: i32,
    pub confidence: f32,
}
static START: Once = Once::new();

pub fn scan_single_image(path: &Path, lang: &str) -> Result<(Vec<OcrLine>, String), Error> {
    let file_bytes = std::fs::read(path).map_err(|_| Error::InternalServerError)?;
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
    api.init("/usr/share/tessdata", lang).map_err(|_| Error::InternalServerError)?;

    // For single regions, PSM 7 (Single line) or PSM 6 (Block) is often better
    api.set_variable("tessedit_pageseg_mode", "6").map_err(|_| Error::InternalServerError)?;

    api.set_image(&raw_data.into_raw(), w as i32, h as i32, 3, (w * 3) as i32)
        .map_err(|_| Error::InternalServerError)?;

    api.recognize().map_err(|_| Error::InternalServerError)?;
    let text = api.get_utf8_text().map_err(|_| Error::InternalServerError)?;

    Ok(text.trim().to_string())
}

// Helper function to prevent code duplication
pub(crate) fn run_tesseract_engine(img_bytes: &[u8], lang: &str, psm: &str) -> Result<(Vec<OcrLine>, String), Error> {
    let mut api = TesseractAPI::new();
    api.init("/usr/share/tessdata", lang).map_err(|_| Error::InternalServerError)?;

    api.set_variable("tessedit_pageseg_mode", psm).map_err(|_| Error::InternalServerError)?;
    api.set_variable("preserve_interword_spaces", "1").map_err(|_| Error::InternalServerError)?;

    let img = image::load_from_memory(img_bytes)
        .map_err(|e| Error::BadRequest(serde_json::json!({"error": e.to_string()})))?;

    let (w, h) = img.dimensions();
    let raw_data = img.to_rgb8();

    api.set_image(&raw_data.into_raw(), w as i32, h as i32, 3, (w * 3) as i32)
        .map_err(|_| Error::InternalServerError)?;

    api.recognize().map_err(|_| Error::InternalServerError)?;
    let full_text = api.get_utf8_text().map_err(|_| Error::InternalServerError)?;

    let ocr_lines = full_text.lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(i, line)| OcrLine {
            text: line.trim().to_string(),
            y_pos: i as i32,
            height: 10,
            confidence: 0.9,
        })
        .collect();

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
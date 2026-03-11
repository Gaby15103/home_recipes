use crate::errors::Error;
use crate::recipe_parser::scanner::tesseract::scan_single_image;
use sqlx::SqlitePool;
use std::path::Path;
use serde_derive::{Deserialize, Serialize};
use regex::Regex;

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
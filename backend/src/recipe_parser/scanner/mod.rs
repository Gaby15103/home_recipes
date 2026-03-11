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
    let mut t = text
        .replace("mi", "ml")
        .replace("I/2", "1/2")
        .replace("l/4", "1/4")
        .replace(['@', '©', '®', 'ù', 'î', 'Ë'], "");

    // REMOVE LEADING NOISE: Strip single characters followed by dots/spaces at line start
    // Matches patterns like "G 4", "î : 250", "d 30", "E, '"
    let re_junk_prefix = Regex::new(r"(?i)^[a-z0-9\W]{1,3}[:.\s'|]+").unwrap();
    t = re_junk_prefix.replace(&t, "").to_string();

    t.trim().to_string()
}
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
    pool: &SqlitePool,
) -> Result<ScannedDocument, Error> {
    let mut all_lines: Vec<String> = Vec::new();
    let mut raw_text = String::new();

    for path in paths {
        let (lines, text) = scan_single_image(path, lang)?;
        raw_text.push_str(&text);

        for line in lines {
            let cleaned = clean_ocr_typos(line.text);
            if cleaned.is_empty() { continue; }

            // --- SMART STITCHING LOGIC ---
            if let Some(last_line) = all_lines.last() {
                // We pass the RAW cleaned strings to the dynamic check
                if should_merge_dynamic(&cleaned, last_line, pool, lang).await {
                    if let Some(last_mut) = all_lines.last_mut() {
                        // Merge with a space
                        *last_mut = format!("{} {}", last_mut, cleaned);
                        continue;
                    }
                }
            }

            // Duplicate Prevention
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

async fn should_merge_dynamic(
    current: &str,
    previous: &str,
    pool: &SqlitePool,
    lang: &str,
) -> bool {
    let prev_lower = previous.to_lowercase();

    // 1. HARD OVERRIDE: Suffix Connectors
    // If the previous line ends with 'et', 'de', 'd'', or a comma,
    // it's a 99% chance the next line is a continuation.
    let trimmed_prev = prev_lower.trim_end_matches([' ', '.', ':', ';']);
    if trimmed_prev.ends_with(" et") ||
        trimmed_prev.ends_with(" de") ||
        trimmed_prev.ends_with(" d'") ||
        trimmed_prev.ends_with(',') {
        return true;
    }

    // 2. Fragment Check:
    // If current line has digits (like '500g'), it's usually a NEW ingredient.
    if current.chars().any(|c| c.is_ascii_digit()) {
        return false;
    }

    // 3. Database Lexicon Check (Descriptors/Actions)
    // Checks if the first word is something like "désossée" or "haché"
    if let Some(first_word) = current.split_whitespace().next() {
        let clean_first = first_word.to_lowercase().replace(['.', ',', ':', ';'], "");

        let is_culinary_continuation: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM lexicon WHERE (term_fr = ? OR term_en = ?) AND category IN ('descriptor', 'action')"
        )
            .bind(&clean_first)
            .bind(&clean_first)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

        if is_culinary_continuation > 0 { return true; }
    }

    // 4. Case-based fallback
    // If it starts with lowercase, merge it.
    current.chars().next().map(|c| c.is_lowercase()).unwrap_or(false)
}

fn clean_ocr_typos(text: String) -> String {
    // Basic cleaning
    let mut t = text
        .replace("mi", "ml")
        .replace("I/2", "1/2")
        .replace("l/4", "1/4")
        .replace(['@', '©', '®', 'ù', 'î', 'Ë', 'H'], "");

    // Remove leading garbage characters often found in French recipes (bullets/OCR noise)
    // Matches patterns like "f :", "e .", "1. ", "î :"
    let re_noise = Regex::new(r"^[a-zA-Z0-9îË] [:.\s]+").unwrap();
    t = re_noise.replace(&t, "").to_string();

    t.trim().to_string()
}
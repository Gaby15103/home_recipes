use std::path::Path;
use crate::errors::Error;
use crate::recipe_parser::scanner::tesseract::scan_single_image;

pub mod tesseract;
pub struct ScannedDocument {
    pub raw_lines: Vec<String>,
    pub detected_lang: String,
}

pub async fn process_batch(paths: Vec<&Path>, lang: &str) -> Result<ScannedDocument, Error> {
    let mut all_lines = Vec::new();

    for path in paths {
        // 1. Get lines from one image
        let lines = scan_single_image(path, lang)?;

        // 2. Pre-process text (Fix OCR hallucinations)
        for mut line in lines {
            line.text = clean_ocr_typos(line.text);

            // 3. Duplicate Prevention (Basic Stitching)
            // If the last line of the previous image is identical to the first of this one, skip
            if let Some(last) = all_lines.last() {
                if last == &line.text { continue; }
            }

            all_lines.push(line.text);
        }
    }

    Ok(ScannedDocument {
        raw_lines: all_lines,
        detected_lang: lang.to_string(),
    })
}

fn clean_ocr_typos(text: String) -> String {
    // Basic grammar fixes at the character level
    text.replace("I/2", "1/2")
        .replace("l/4", "1/4")
        .replace("|", "")
        .trim()
        .to_string()
}
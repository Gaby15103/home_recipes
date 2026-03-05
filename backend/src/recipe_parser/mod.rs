use std::time::Instant;
use std::path::Path;
use crate::errors::Error;
use crate::dto::recipe_dto::CreateRecipeInput;
use crate::dto::unit_dto::UnitDto;
use sqlx::SqlitePool;
use crate::dto::recipe_ocr::{ConfirmIngredient, OcrConfirmInput, OcrCorrectionWrapper, OcrResultResponse};

pub mod dictionary;
pub mod grammar;
mod scanner;
mod classifier;

/*
  TODO: ENHANCEMENTS FOR RECIPE PARSER
  ---------------------------------------------------------------------
  CURRENT STATUS:
  - Successfully handling Multipart uploads and Tesseract memory buffers.
  - Basic Lexicon matching for units (ml, g) and core ingredients (shrimp) is active.

  PLANNED IMPROVEMENTS:
  1. PRE-PROCESSOR:
     - Add a 'Denoise' pass to strip OCR artifacts (Ë, î, %, #) from line starts.
     - Implement regex to handle "vulgar fractions" (½, ¼) which Tesseract often chokes on.

  2. LEXICON EXPANSION:
     - Add French aliases for base aromatics (ail, oignon, échalote).
     - Map 'huile végétale' and 'poivron' to the dictionary to eliminate `ingredient: null`.

  3. HEURISTIC RECOVERY:
     - Logic: If an 'Ingredient Group Name' contains a numeric quantity, re-classify
       the group name as a standalone ingredient (fixes "4 gousses d'ail" being a header).

  4. SEMANTIC LINKING:
     - Cross-reference `detected_actions` (hachées, taillés) with the `dictionary.db`
       to populate the `actions` array instead of leaving it empty.
*/

/// A clean way to pass dependencies into the parser engine
pub struct ParserContext<'a> {
    pub sqlite_pool: &'a SqlitePool,
    pub known_units: Vec<UnitDto>,
}

pub async fn run_pipeline(
    paths: &[&Path],
    ctx: ParserContext<'_>
) -> Result<OcrResultResponse, Error> {
    let total_start = Instant::now();

    let scan_start = Instant::now();
    let document = scanner::process_batch(Vec::from(paths), "eng+fra").await?;
    let scan_duration = scan_start.elapsed();

    let classify_start = Instant::now();
    let mut classifier = classifier::DocumentClassifier::new();
    let classified_lines = classifier.segment_document(document);
    let classify_duration = classify_start.elapsed();

    // 3. Grammar & Dictionary: The "Brain" phase
    // We pass the classified_lines here to be assembled into the final DTO
    let dict_start = Instant::now();
    let ocr_result = grammar::assemble_recipe(classified_lines, ctx.sqlite_pool).await?;
    let dict_duration = dict_start.elapsed();

    // --- 🛠️ Feedback Output ---
    println!("--- 🛠️ Recipe Parser Feedback ---");
    println!("📸 OCR Scan (Batch): {:.2?}", scan_duration);
    println!("🗂️ Classification:   {:.2?}", classify_duration);
    println!("📖 Lexicon Match:    {:.2?}", dict_duration);
    println!("🚀 Total Pipeline:   {:.2?}", total_start.elapsed());
    println!("---------------------------------");

    Ok(ocr_result)
}
pub async fn teach_lexicon(wrapper: &OcrCorrectionWrapper, pool: &SqlitePool) -> Result<(), Error> {
    // We iterate through the original OCR results to get the 'raw_text'
    // and match them by position/index to the user's 'confirmed_lexicon_id'
    for ocr_group in &wrapper.original_ocr.ingredient_groups {
        for ocr_line in &ocr_group.ingredients {

            // Look for the matching ingredient in the modified input
            if let Some(confirmed) = find_confirmed_ingredient(wrapper, ocr_line.position) {
                let raw_text = ocr_line.original_line.to_lowercase().trim().to_string();

                // If it's not empty, update the SQLite Brain
                if !raw_text.is_empty() {
                    sqlx::query(
                        r#"
                        INSERT INTO aliases (raw_text, lexicon_id, confidence)
                        VALUES (?, ?, 1.0)
                        ON CONFLICT(raw_text) DO UPDATE SET lexicon_id = excluded.lexicon_id
                        "#
                    )
                        .bind(raw_text)
                        .bind(confirmed.confirmed_lexicon_id)
                        .execute(pool)
                        .await
                        .ok(); // Log error but don't crash if learning fails
                }
            }
        }
    }
    Ok(())
}
fn find_confirmed_ingredient(wrapper: &OcrCorrectionWrapper, pos: i32) -> Option<&ConfirmIngredient> {
    wrapper.modified_recipe.ingredient_groups.iter()
        .flat_map(|g| &g.ingredients)
        .find(|i| i.position == pos)
}
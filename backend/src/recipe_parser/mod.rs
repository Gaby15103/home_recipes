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
    let document = scanner::process_batch(Vec::from(paths), "eng+fra", ctx.sqlite_pool).await?;
    let scan_duration = scan_start.elapsed();

    let classify_start = Instant::now();
    let mut classifier = classifier::DocumentClassifier::new(&ctx.known_units, ctx.sqlite_pool);
    let classified_lines = classifier.segment_document(document.clone()).await;
    let classify_duration = classify_start.elapsed();

    // 3. Grammar & Dictionary: The "Brain" phase
    // We pass the classified_lines here to be assembled into the final DTO
    let dict_start = Instant::now();
    let ocr_result = grammar::assemble_recipe(classified_lines, ctx.sqlite_pool, document.raw_text).await?;
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
    // 1. Process explicit feedback from the "Corrections" list
    // This is for things like "h ile" -> "Huile"
    for correction in &wrapper.lexicon_feedback {
        let clean_token = correction.raw_token.to_lowercase().trim().to_string();

        if !clean_token.is_empty() {
            sqlx::query(
                r#"
                INSERT INTO aliases (raw_text, lexicon_id, confidence)
                VALUES (?, ?, 1.0)
                ON CONFLICT(raw_text) DO UPDATE SET lexicon_id = excluded.lexicon_id
                "#
            )
                .bind(clean_token)
                .bind(correction.lexicon_id)
                .execute(pool)
                .await?;
        }
    }

    // 2. Process the Ingredient Groups to catch merged lines
    // This handles the "Poulet" + "Désossée" -> Chicken ID mapping
    for group in &wrapper.modified_recipe.ingredient_groups {
        for ing in &group.ingredients {
            // If the user associated a Lexicon ID with these lines
            if let Some(lex_id) = ing.confirmed_lexicon_id {
                for raw_line in &ing.source_ocr_lines {
                    let clean_line = raw_line.to_lowercase().trim().to_string();

                    if !clean_line.is_empty() {
                        sqlx::query(
                            "INSERT INTO aliases (raw_text, lexicon_id, confidence)
                             VALUES (?, ?, 0.9)
                             ON CONFLICT(raw_text) DO NOTHING"
                        )
                            .bind(clean_line)
                            .bind(lex_id)
                            .execute(pool)
                            .await
                            .ok(); // We use .ok() here because we don't want to fail the whole save if one alias exists
                    }
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
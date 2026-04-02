use std::time::Instant;
use std::path::Path;
use actix_multipart::form::tempfile::TempFile;
use regex::Regex;
use crate::errors::Error;
use crate::dto::recipe_dto::CreateRecipeInput;
use crate::dto::unit_dto::UnitDto;
use sqlx::SqlitePool;
use crate::dto::ingredient_dto::IngredientInput;
use crate::dto::recipe_ocr::{ConfirmIngredient, OcrConfirmInput, OcrCorrectionWrapper, OcrResultResponse};
use crate::dto::upload_dto::RegionDto;
use crate::recipe_parser::classifier::{ClassifiedLine, LineType};
use crate::recipe_parser::scanner::ScannedDocument;

pub mod dictionary;
pub mod grammar;
mod scanner;
mod classifier;
mod translator;
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
    let ocr_result = grammar::assemble_recipe(classified_lines, ctx.sqlite_pool, document.raw_text, document.detected_lang).await?;
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
pub async fn run_region_pipeline(
    images: Vec<TempFile>,
    mut regions: Vec<RegionDto>,
    lang: &str,
    ctx: ParserContext<'_>
) -> Result<OcrResultResponse, Error> {
    regions.sort_by(|a, b| {
        a.image_index.cmp(&b.image_index)
            .then(a.y.cmp(&b.y))
            .then(a.x.cmp(&b.x))
    });

    let mut classified_lines = Vec::new();
    let mut raw_text_acc = String::new();
    let mut classifier = classifier::DocumentClassifier::new(&ctx.known_units, ctx.sqlite_pool);

    for (i, region) in regions.into_iter().enumerate() {
        let temp_file = images.get(region.image_index)
            .ok_or_else(|| Error::BadRequest(serde_json::json!({"error": "Index image invalid"})))?;

        let region_text = scanner::scan_region(temp_file.file.path(), &region, lang)?;
        if region_text.is_empty() { continue; }

        raw_text_acc.push_str(&region_text);
        raw_text_acc.push('\n');

        // Logic to buffer ingredients that span multiple lines
        let mut current_buffer: Option<String> = None;

        // Dans mod.rs (run_region_pipeline)

        for line in region_text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }

            // Utilise la regex plus permissive qu'on a vue (chiffre + n'importe quel signe)
            let is_new_step = region.label == "steps" && Regex::new(r"^(\d+[\.\),\s]|[-•*])").unwrap().is_match(trimmed);
            let is_new_ing = region.label == "ingredients" && trimmed.chars().next().map_or(false, |c| c.is_numeric() || "¼½¾⅓⅔⅛⅜⅝⅞".contains(c));

            if is_new_step || is_new_ing {
                // 1. On vide ce qu'il y avait avant
                if let Some(prev) = current_buffer {
                    classified_lines.push(classifier.classify_labeled_region(prev, &region.label, i).await);
                }
                // 2. On commence le nouveau buffer
                current_buffer = Some(trimmed.to_string());
            } else {
                // C'est une continuation (comme la fin de l'étape 6 qui n'a pas de chiffre)
                if let Some(ref mut buffer) = current_buffer {
                    buffer.push(' ');
                    buffer.push_str(trimmed);
                } else {
                    // Cas de secours : si on a du texte sans buffer au début d'une région
                    current_buffer = Some(trimmed.to_string());
                }
            }
        }

        // Push the final buffered ingredient for this region
        if let Some(last_buffered) = current_buffer {
            // ADD .await HERE
            classified_lines.push(classifier.classify_labeled_region(last_buffered, &region.label, i).await);
        }
    }

    let ocr_result = grammar::assemble_recipe(
        classified_lines,
        ctx.sqlite_pool,
        raw_text_acc,
        lang.to_string()
    ).await?;

    Ok(ocr_result)
}

fn manual_to_classified(title: String, ingredients: String, steps: String) -> Vec<ClassifiedLine> {
    let mut results = Vec::new();
    let mut global_idx = 0;

    // 1. Process Title
    if !title.trim().is_empty() {
        results.push(ClassifiedLine {
            raw_text: title.trim().to_string(),
            line_type: LineType::Title,
            index: global_idx,
            confidence: 1.0,
        });
        global_idx += 1;
    }

    // 2. Process Ingredients
    for line in ingredients.lines().filter(|l| !l.trim().is_empty()) {
        results.push(ClassifiedLine {
            raw_text: line.to_string(),
            line_type: LineType::Ingredient,
            index: global_idx,
            confidence: 1.0,
        });
        global_idx += 1;
    }

    // 3. Process Steps
    for line in steps.lines().filter(|l| !l.trim().is_empty()) {
        results.push(ClassifiedLine {
            raw_text: line.to_string(),
            line_type: LineType::Instruction,
            index: global_idx,
            confidence: 1.0,
        });
        global_idx += 1;
    }

    results
}
pub async fn teach_lexicon(wrapper: &OcrCorrectionWrapper, pool: &SqlitePool) -> Result<(), Error> {
    // 1. Process explicit feedback from the "Corrections" list
    // This remains mostly the same, as it's independent of the recipe structure
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

    // 2. IMPORTANT: If you want to catch "merged lines" from the new structure,
    // you have to look at the translations data in CreateRecipeInput.
    // However, since CreateRecipeInput is "clean", it usually doesn't
    // contain the raw source lines anymore unless you put them in the 'note' or 'data'.

    // If your frontend still sends "unconfirmed" raw strings in the 'data' field
    // of translations for new ingredients, you could loop through them here.
    // Otherwise, the explicit `lexicon_feedback` loop above is your primary teacher.

    Ok(())
}

// Updated helper to find an ingredient in the NEW CreateRecipeInput structure
fn find_confirmed_ingredient(wrapper: &OcrCorrectionWrapper, pos: i32) -> Option<&IngredientInput> {
    wrapper.modified_recipe.ingredient_groups.iter()
        .flat_map(|g| &g.ingredients)
        .find(|i| i.position == pos)
}
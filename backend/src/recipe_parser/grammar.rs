use crate::recipe_parser::classifier::{ClassifiedLine, LineType};
use crate::recipe_parser::dictionary;
use crate::dto::recipe_ocr::{OcrResultResponse, ParsedIngredientLine, OcrStep, OcrIngredientGroup, OcrStepGroup};
use sqlx::SqlitePool;
use crate::errors::Error;
use regex::Regex;

pub async fn assemble_recipe(
    classified_lines: Vec<ClassifiedLine>,
    pool: &SqlitePool,
    raw_text: String,
) -> Result<OcrResultResponse, Error> {
    let mut ingredient_groups = Vec::new();
    let mut step_groups = Vec::new();
    let mut unparsed_segments = Vec::new();
    let mut title = None;
    let mut detected_servings = None;

    // Regex to catch "6 PORTIONS" or "Serves 4"
    let servings_re = Regex::new(r"(?i)(\d+)\s*(portions|servings|personnes|serves)").unwrap();

    let mut current_ing_group = OcrIngredientGroup {
        name: "Ingredients".into(),
        ingredients: Vec::new(),
    };
    let mut current_step_group = OcrStepGroup {
        name: "Instructions".into(),
        steps: Vec::new(),
    };

    for line in classified_lines {
        let text = line.raw_text.trim();
        if text.is_empty() { continue; }

        // 1. Check for Servings/Metadata first
        if let Some(caps) = servings_re.captures(text) {
            if let Some(num) = caps.get(1).map(|m| m.as_str().parse::<i32>().unwrap_or(0)) {
                detected_servings = Some(num);
                continue; // Skip adding this as an ingredient/step
            }
        }

        match line.line_type {
            LineType::Title => {
                if title.is_none() { title = Some(text.to_string()); }
            }

            LineType::Header => {
                // Logic: Only switch groups if the header isn't just noise
                if !current_ing_group.ingredients.is_empty() {
                    ingredient_groups.push(current_ing_group.clone());
                    current_ing_group.ingredients.clear();
                }
                current_ing_group.name = text.to_string();

                if !current_step_group.steps.is_empty() {
                    step_groups.push(current_step_group.clone());
                    current_step_group.steps.clear();
                }
                current_step_group.name = text.to_string();
            }

            LineType::Ingredient => {
                // --- MULTI-LINE MERGE LOGIC ---
                // If this line has NO quantity and the previous line was an ingredient,
                // we merge it instead of creating a new entry.
                let (qty, unit, ing, actions) = dictionary::resolve_line(text, pool).await?;

                if qty.is_none() && !current_ing_group.ingredients.is_empty() {
                    if let Some(last) = current_ing_group.ingredients.last_mut() {
                        last.original_line = format!("{} {}", last.original_line, text);
                        // Re-resolve the merged line to see if we found a match now
                        let (_, _, new_ing, _) = dictionary::resolve_line(&last.original_line, pool).await?;
                        last.ingredient = new_ing;
                        continue;
                    }
                }

                current_ing_group.ingredients.push(ParsedIngredientLine {
                    quantity: qty,
                    unit,
                    ingredient: ing,
                    actions,
                    original_line: text.to_string(),
                    position: line.index as i32,
                });
            }

            LineType::Instruction => {
                current_step_group.steps.push(OcrStep {
                    position: line.index as i32,
                    raw_text: text.to_string(),
                    detected_actions: vec![],
                    detected_equipment: vec![],
                });
            }

            LineType::Fluff => {
                unparsed_segments.push(text.to_string());
            }
        }
    }

    // Final Push
    if !current_ing_group.ingredients.is_empty() { ingredient_groups.push(current_ing_group); }
    if !current_step_group.steps.is_empty() { step_groups.push(current_step_group); }

    Ok(OcrResultResponse {
        primary_language: "fr".into(),
        title,
        detected_servings,
        ingredient_groups,
        step_groups,
        unparsed_segments,
        raw_text
    })
}
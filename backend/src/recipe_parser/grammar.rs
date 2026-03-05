use crate::recipe_parser::classifier::{ClassifiedLine, LineType};
use crate::recipe_parser::dictionary;
use crate::dto::recipe_ocr::{OcrResultResponse, ParsedIngredientLine, OcrStep, OcrIngredientGroup, OcrStepGroup};
use sqlx::SqlitePool;
use crate::errors::Error;

/// The Final Assembler: Converts classified lines into structured DTOs
/// by resolving ingredients against the Lexicon.
pub async fn assemble_recipe(
    classified_lines: Vec<ClassifiedLine>,
    pool: &SqlitePool,
) -> Result<OcrResultResponse, Error> {
    let mut ingredient_groups = Vec::new();
    let mut step_groups = Vec::new();
    let mut title = None;

    // Initialize default groups
    let mut current_ing_group = OcrIngredientGroup {
        name: "Ingredients".into(),
        ingredients: Vec::new(),
    };
    let mut current_step_group = OcrStepGroup {
        name: "Instructions".into(),
        steps: Vec::new(),
    };

    for line in classified_lines {
        match line.line_type {
            LineType::Title => {
                if title.is_none() {
                    title = Some(line.raw_text);
                }
            }

            LineType::Header => {
                // When we hit a header, check if the previous groups have content.
                // If they do, push them and start fresh ones with the header name.
                if !current_ing_group.ingredients.is_empty() {
                    ingredient_groups.push(current_ing_group);
                }
                if !current_step_group.steps.is_empty() {
                    step_groups.push(current_step_group);
                }

                current_ing_group = OcrIngredientGroup {
                    name: line.raw_text.clone(),
                    ingredients: Vec::new(),
                };
                current_step_group = OcrStepGroup {
                    name: line.raw_text,
                    steps: Vec::new(),
                };
            }

            LineType::Ingredient => {
                let (qty, unit, ing, actions) = dictionary::resolve_line(&line.raw_text, pool).await?;

                current_ing_group.ingredients.push(ParsedIngredientLine {
                    quantity: qty,
                    unit,
                    ingredient: ing,
                    actions,
                    original_line: line.raw_text,
                    position: line.index as i32,
                });
            }

            LineType::Instruction => {
                current_step_group.steps.push(OcrStep {
                    position: line.index as i32,
                    raw_text: line.raw_text,
                    detected_actions: vec![],
                    detected_equipment: vec![],
                });
            }

            LineType::Fluff => continue,
        }
    }

    // Push the final groups if they contain any data
    if !current_ing_group.ingredients.is_empty() {
        ingredient_groups.push(current_ing_group);
    }
    if !current_step_group.steps.is_empty() {
        step_groups.push(current_step_group);
    }

    Ok(OcrResultResponse {
        primary_language: "en".into(),
        title,
        ingredient_groups,
        step_groups,
    })
}
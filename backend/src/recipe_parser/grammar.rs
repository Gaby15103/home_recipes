use crate::recipe_parser::classifier::{ClassifiedLine, LineType};
use crate::recipe_parser::dictionary;
use crate::dto::recipe_ocr::{OcrResultResponse, ParsedIngredientLine, OcrStep, OcrIngredientGroup, OcrStepGroup};
use sqlx::SqlitePool;
use crate::errors::Error;

pub async fn assemble_recipe(
    classified_lines: Vec<ClassifiedLine>,
    pool: &SqlitePool,
    raw_text: String,
) -> Result<OcrResultResponse, Error> {
    let mut ingredient_groups = Vec::new();
    let mut step_groups = Vec::new();
    let mut unparsed_segments = Vec::new();
    let mut title = None;

    // Initialize with default groups
    let mut current_ing_group = OcrIngredientGroup { name: "Ingredients".into(), ingredients: Vec::new() };
    let mut current_step_group = OcrStepGroup { name: "Instructions".into(), steps: Vec::new() };

    for line in classified_lines {
        let text = line.raw_text.trim();
        if text.is_empty() { continue; }

        match line.line_type {
            LineType::Title => {
                if title.is_none() { title = Some(text.to_string()); }
            }

            LineType::Header => {
                let lower = text.to_lowercase();

                // DATA-DRIVEN CHECK: Is this actually an instruction header?
                let is_step_trigger: i32 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM lexicon WHERE (term_en = ? OR term_fr = ?) AND category = 'text' AND (term_en LIKE '%instruction%' OR term_fr LIKE '%prépar%')"
                ).bind(&lower).bind(&lower).fetch_one(pool).await.unwrap_or(0);

                if is_step_trigger > 0 {
                    if !current_step_group.steps.is_empty() {
                        step_groups.push(current_step_group.clone());
                    }
                    current_step_group = OcrStepGroup { name: text.to_string(), steps: Vec::new() };
                } else {
                    // Only start a new ingredient group if the current one isn't the default or actually has content
                    // This prevents "Ingredients" followed immediately by "4 EN INGREDIENTS" from splitting
                    if !current_ing_group.ingredients.is_empty() {
                        ingredient_groups.push(current_ing_group.clone());
                    }
                    current_ing_group = OcrIngredientGroup { name: text.to_string(), ingredients: Vec::new() };
                }
            }

            LineType::Ingredient => {
                // 1. Initial resolution of the current line
                let (qty, unit, ing, actions) = dictionary::resolve_line(text, pool).await?;

                let mut merged = false;

                // 2. Check if this line should be merged into the previous ingredient
                if !current_ing_group.ingredients.is_empty() {
                    let last = current_ing_group.ingredients.last_mut().unwrap();
                    let last_raw = last.original_line.to_lowercase();
                    let current_raw = text.to_lowercase();

                    // Check DB-defined continuation rules (e.g., lines ending in "," or "et")
                    let is_suffix: i32 = sqlx::query_scalar(
                        "SELECT COUNT(*) FROM line_continuation_rules WHERE rule_type = 'SUFFIX' AND ? LIKE '%' || pattern"
                    ).bind(&last_raw).fetch_one(pool).await.unwrap_or(0);

                    let is_prefix: i32 = sqlx::query_scalar(
                        "SELECT COUNT(*) FROM line_continuation_rules WHERE rule_type = 'PREFIX' AND ? LIKE pattern || '%'"
                    ).bind(&current_raw).fetch_one(pool).await.unwrap_or(0);

                    // HEURISTIC RECOVERY:
                    // If the current line has no quantity AND no known ingredient, it's almost certainly
                    // a trailing descriptor (like "haché") for the previous item.
                    let is_orphan_descriptor = qty.is_none() && ing.is_none();

                    if is_suffix > 0 || is_prefix > 0 || is_orphan_descriptor {
                        // Merge the text and re-run resolution on the combined string
                        // to catch multi-word matches (e.g., "pomme" + "de terre")
                        last.original_line = format!("{} {}", last.original_line, text);

                        let (nq, nu, ni, na) = dictionary::resolve_line(&last.original_line, pool).await?;

                        last.quantity = nq;
                        last.unit = nu;
                        last.ingredient = ni;
                        last.actions = na;
                        merged = true;
                    }
                }

                // 3. If not a continuation, treat as a fresh ingredient line
                if !merged {
                    current_ing_group.ingredients.push(ParsedIngredientLine {
                        quantity: qty,
                        unit,
                        ingredient: ing,
                        actions,
                        original_line: text.to_string(),
                        position: line.index as i32,
                    });
                }
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

    // Final push of groups
    if !current_ing_group.ingredients.is_empty() { ingredient_groups.push(current_ing_group); }
    if !current_step_group.steps.is_empty() { step_groups.push(current_step_group); }

    // DEDUPLICATION: If we have multiple groups and the first one is "Ingredients"
    // and the second one is also a variation of "Ingredients", merge them.
    if ingredient_groups.len() > 1 {
        let first_name = ingredient_groups[0].name.to_lowercase();
        let second_name = ingredient_groups[1].name.to_lowercase();
        if first_name == "ingredients" && second_name.contains("ingred") {
            let mut second_group = ingredient_groups.remove(1);
            ingredient_groups[0].ingredients.append(&mut second_group.ingredients);
        }
    }

    Ok(OcrResultResponse {
        primary_language: "fr".into(),
        title,
        detected_servings: None, // Logic for this should be added to dictionary or separate util
        ingredient_groups,
        step_groups,
        unparsed_segments,
        raw_text
    })
}
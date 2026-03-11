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

    // Regex for: "6 portions", "Pour 4 personnes", "Serves 4", etc.
    let serving_re = Regex::new(r"(?i)(\d+)\s*(portions?|personnes?|servings?|serves)").unwrap();

    let mut current_ing_group = OcrIngredientGroup { name: "Ingrédients".into(), ingredients: Vec::new() };
    let mut current_step_group = OcrStepGroup { name: "Préparation".into(), steps: Vec::new() };

    for line in classified_lines {
        let text = line.raw_text.trim();
        if text.is_empty() { continue; }

        match line.line_type {
            LineType::Title => {
                if title.is_none() { title = Some(text.to_string()); }
            }

            LineType::Header => {
                let lower = text.to_lowercase();
                let is_step_trigger: i32 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM lexicon WHERE category = 'text' AND (term_en LIKE '%instruction%' OR term_fr LIKE '%prépar%') AND (term_en = ? OR term_fr = ?)"
                ).bind(&lower).bind(&lower).fetch_one(pool).await.unwrap_or(0);

                if is_step_trigger > 0 {
                    if !current_step_group.steps.is_empty() { step_groups.push(current_step_group.clone()); }
                    current_step_group = OcrStepGroup { name: text.to_string(), steps: Vec::new() };
                } else {
                    if !current_ing_group.ingredients.is_empty() { ingredient_groups.push(current_ing_group.clone()); }
                    current_ing_group = OcrIngredientGroup { name: text.to_string(), ingredients: Vec::new() };
                }
            }

            LineType::Ingredient => {
                let lower = text.to_lowercase();

                // 1. Check for Servings (and prevent them from becoming ingredients)
                if let Some(caps) = serving_re.captures(&lower) {
                    if let Some(val) = caps.get(1) {
                        detected_servings = val.as_str().parse::<i32>().ok();
                        continue;
                    }
                }

                // Updated to receive the cleaned display_name from resolve_line
                let (qty, unit, mut ing, actions, cleaned_name) = dictionary::resolve_line(text, pool).await?;

                // 2. Filter out "Hallucinations"
                if let Some(ref i) = ing {
                    if i.raw_token.len() <= 2 && qty.is_none() {
                        ing = None;
                    }
                }

                let mut merged = false;
                if !current_ing_group.ingredients.is_empty() {
                    let last = current_ing_group.ingredients.last_mut().unwrap();
                    let last_raw = last.original_line.to_lowercase();
                    let current_raw = text.to_lowercase();

                    let is_suffix: i32 = sqlx::query_scalar(
                        "SELECT COUNT(*) FROM line_continuation_rules WHERE rule_type = 'SUFFIX' AND ? LIKE '%' || pattern"
                    ).bind(&last_raw).fetch_one(pool).await.unwrap_or(0);

                    let is_prefix: i32 = sqlx::query_scalar(
                        "SELECT COUNT(*) FROM line_continuation_rules WHERE rule_type = 'PREFIX' AND ? LIKE pattern || '%'"
                    ).bind(&current_raw).fetch_one(pool).await.unwrap_or(0);

                    if is_suffix > 0 || is_prefix > 0 || (qty.is_none() && ing.is_none()) {
                        last.original_line = format!("{} {}", last.original_line, text);

                        // Re-resolve the merged line and update all fields including display_name
                        let (nq, nu, ni, na, n_dn) = dictionary::resolve_line(&last.original_line, pool).await?;
                        last.quantity = nq;
                        last.unit = nu;
                        last.ingredient = ni;
                        last.actions = na;
                        last.display_name = n_dn;
                        merged = true;
                    }
                }

                if !merged {
                    current_ing_group.ingredients.push(ParsedIngredientLine {
                        quantity: qty,
                        unit,
                        ingredient: ing,
                        actions,
                        original_line: text.to_string(),
                        display_name: cleaned_name,
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

            _ => { unparsed_segments.push(text.to_string()); }
        }
    }

    if !current_ing_group.ingredients.is_empty() { ingredient_groups.push(current_ing_group); }
    if !current_step_group.steps.is_empty() { step_groups.push(current_step_group); }

    if ingredient_groups.len() > 1 {
        let name = ingredient_groups[0].name.to_lowercase();
        if name == "ingrédients" || name == "ingredients" {
            let mut second = ingredient_groups.remove(1);
            ingredient_groups[0].ingredients.append(&mut second.ingredients);
        }
    }

    Ok(OcrResultResponse {
        primary_language: "fr".into(),
        title,
        detected_servings,
        ingredient_groups,
        step_groups,
        unparsed_segments,
        raw_text,
    })
}
use crate::recipe_parser::classifier::{ClassifiedLine, LineType};
use crate::recipe_parser::{classifier, dictionary, translator};
use crate::dto::recipe_ocr::{OcrResultResponse, ParsedIngredientLine, OcrStep, OcrIngredientGroup, OcrStepGroup};
use sqlx::SqlitePool;
use crate::errors::Error;
use regex::Regex;

pub async fn assemble_recipe(
    classified_lines: Vec<ClassifiedLine>,
    pool: &SqlitePool,
    raw_text: String,
    source_lang: String,
) -> Result<OcrResultResponse, Error> {
    let mut ingredient_groups = Vec::new();
    let mut step_groups = Vec::new();
    let mut unparsed_segments = Vec::new();

    let mut title_en = String::new();
    let mut title_fr = String::new();
    let mut detected_servings = None;

    let mut current_ing_group = OcrIngredientGroup {
        name_en: "Ingredients".into(),
        name_fr: "Ingrédients".into(),
        ingredients: Vec::new()
    };
    let mut current_step_group = OcrStepGroup {
        name_en: "Preparation".into(),
        name_fr: "Préparation".into(),
        steps: Vec::new()
    };

    let mut ing_buffer: Vec<String> = Vec::new();
    let mut step_buffer: Vec<String> = Vec::new();
    let mut last_ing_index = 0;
    let mut last_step_index = 0;

    for line in classified_lines {
        let text = line.raw_text.trim();
        if text.is_empty() { continue; }

        match line.line_type {
            LineType::Title => {
                if title_en.is_empty() && title_fr.is_empty() {
                    title_en = if source_lang == "fr" { translator::translate_text(text, "fr", "en").await? } else { text.to_string() };
                    title_fr = if source_lang == "en" { translator::translate_text(text, "en", "fr").await? } else { text.to_string() };
                }
            }

            LineType::Header => {
                flush_ing_buffer(&mut ing_buffer, &mut current_ing_group, last_ing_index, pool, &source_lang).await?;
                flush_step_buffer(&mut step_buffer, &mut current_step_group, last_step_index, &source_lang).await?;

                let h_en = if source_lang == "fr" { translator::translate_text(text, "fr", "en").await? } else { text.to_string() };
                let h_fr = if source_lang == "en" { translator::translate_text(text, "en", "fr").await? } else { text.to_string() };

                if is_step_header(text) {
                    if !current_step_group.steps.is_empty() { step_groups.push(current_step_group.clone()); }
                    current_step_group = OcrStepGroup { name_en: h_en, name_fr: h_fr, steps: Vec::new() };
                } else {
                    if !current_ing_group.ingredients.is_empty() { ingredient_groups.push(current_ing_group.clone()); }
                    current_ing_group = OcrIngredientGroup { name_en: h_en, name_fr: h_fr, ingredients: Vec::new() };
                }
            }

            LineType::Ingredient => {
                if (starts_with_quantity(text) || starts_with_vulgar_fraction(text)) && !ing_buffer.is_empty() {
                    flush_ing_buffer(&mut ing_buffer, &mut current_ing_group, last_ing_index, pool, &source_lang).await?;
                }
                if ing_buffer.is_empty() { last_ing_index = line.index; }
                ing_buffer.push(text.to_string());
            }

            LineType::Instruction => {
                if starts_with_step_indicator(text) && !step_buffer.is_empty() {
                    flush_step_buffer(&mut step_buffer, &mut current_step_group, last_step_index, &source_lang).await?;
                }
                if step_buffer.is_empty() { last_step_index = line.index; }
                step_buffer.push(text.to_string());
            }

            _ => { unparsed_segments.push(text.to_string()); }
        }
    }

    flush_ing_buffer(&mut ing_buffer, &mut current_ing_group, last_ing_index, pool, &source_lang).await?;
    flush_step_buffer(&mut step_buffer, &mut current_step_group, last_step_index, &source_lang).await?;

    if !current_ing_group.ingredients.is_empty() { ingredient_groups.push(current_ing_group); }
    if !current_step_group.steps.is_empty() { step_groups.push(current_step_group); }

    Ok(OcrResultResponse {
        primary_language: source_lang,
        title_en,
        title_fr,
        detected_servings,
        ingredient_groups,
        step_groups,
        unparsed_segments,
        raw_text,
    })
}

async fn flush_ing_buffer(
    buffer: &mut Vec<String>,
    group: &mut OcrIngredientGroup,
    _index: usize,
    pool: &SqlitePool,
    source_lang: &str,
) -> Result<(), Error> {
    if buffer.is_empty() { return Ok(()); }

    let combined = buffer.join(" ");

    let cleaned_content = classifier::DocumentClassifier::clean_ingredient_text(&combined);

    for segment in cleaned_content.lines() {
        let segment = segment.trim();
        if segment.is_empty() { continue; }

        let analysis_text = if source_lang == "en" {
            translator::translate_text(segment, "en", "fr").await?
        } else {
            segment.to_string()
        };

        let (qty, unit, ing, actions, disp_en, disp_fr) =
            dictionary::resolve_line(&analysis_text, pool).await?;

        let final_disp_en = if source_lang == "fr" {
            translator::translate_text(&disp_fr, "fr", "en").await?
        } else {
            disp_en
        };

        group.ingredients.push(ParsedIngredientLine {
            quantity: qty,
            unit,
            ingredient: ing,
            actions,
            original_line: segment.to_string(),
            display_name_en: final_disp_en,
            display_name_fr: disp_fr,
            // FIX: Use length for position
            position: group.ingredients.len() as i32,
        });
    }

    buffer.clear();
    Ok(())
}

async fn flush_step_buffer(
    buffer: &mut Vec<String>,
    group: &mut OcrStepGroup,
    _index: usize,
    source_lang: &str
) -> Result<(), Error> {
    if buffer.is_empty() { return Ok(()); }

    let combined = buffer.join(" ");

    let re_step_split = Regex::new(r"(?m)(?:\s+|^)(\d+[\.\)]|[-•*])\s+").unwrap();

    let mut step_texts = Vec::new();
    let mut last_pos = 0;

    for mat in re_step_split.find_iter(&combined) {
        if mat.start() > last_pos {
            let part = combined[last_pos..mat.start()].trim().to_string();
            if !part.is_empty() {
                step_texts.push(part);
            }
        }
        last_pos = mat.start();
    }
    step_texts.push(combined[last_pos..].trim().to_string());

    for text in step_texts {
        if text.is_empty() { continue; }

        let text_en = if source_lang == "fr" { translator::translate_text(&text, "fr", "en").await? } else { text.clone() };
        let text_fr = if source_lang == "en" { translator::translate_text(&text, "en", "fr").await? } else { text.clone() };

        group.steps.push(OcrStep {
            position: group.steps.len() as i32,
            raw_text_en: text_en,
            raw_text_fr: text_fr,
            detected_actions: Vec::new(),
            detected_equipment: Vec::new(),
        });
    }

    buffer.clear();
    Ok(())
}

fn starts_with_quantity(text: &str) -> bool {
    text.chars().next().map_or(false, |c| c.is_ascii_digit())
}

fn starts_with_vulgar_fraction(text: &str) -> bool {
    text.chars().next().map_or(false, |c| "¼½¾⅓⅔⅛⅜⅝⅞".contains(c))
}

fn starts_with_step_indicator(text: &str) -> bool {
    let re = Regex::new(r"^(\d+[\.\),\s]|[-•*])").unwrap();
    re.is_match(text.trim())
}

fn is_step_header(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("prépar") || lower.contains("étape") || lower.contains("instruc") || lower.contains("method")
}
use crate::dto::recipe_dto::{CreateRecipeInput, RecipeTranslationInput};
use crate::dto::ingredient_group_dto::{IngredientGroupInput, IngredientGroupTranslationInput};
use crate::dto::ingredient_dto::{IngredientInput, IngredientTranslationInput};
use crate::dto::step_group_dto::{StepGroupInput, StepGroupTranslationInput};
use crate::dto::step_dto::{StepInput, StepTranslationInput};
use crate::dto::unit_dto::UnitDto;
use crate::errors::Error;
use migration::prelude::Decimal; // Correct Decimal type
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;
use regex::Regex;

#[derive(Deserialize, Debug)]
struct SimpleExtraction {
    title_fr: String,
    title_en: Option<String>,
    description_fr: Option<String>,
    description_en: Option<String>,
    servings: i32,
    prep_time: i32,
    cook_time: i32,
    ingredients: Vec<SimpleIngredient>,
    steps_fr: Vec<String>,
    steps_en: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct SimpleIngredient {
    name_fr: String,
    name_en: Option<String>,
    quantity: f64,
    unit_symbol: String,
    note_fr: Option<String>,
}

pub async fn process_ocr_to_dto(
    ocr_text: &str,
    units: Vec<UnitDto>,
) -> Result<CreateRecipeInput, Error> {
    let client = reqwest::Client::new();
    let cleaned_text = clean_ocr(ocr_text);

    let mut unit_map: HashMap<String, Uuid> = HashMap::new();
    for u in &units {
        unit_map.insert(u.symbol.to_lowercase(), u.id);
        unit_map.insert(u.name_fr.to_lowercase(), u.id);
        unit_map.insert(u.name_en.to_lowercase(), u.id);
    }

    let system_instructions = r#"Extract recipe. JSON ONLY.
    - unit_symbol: use 'g', 'ml', 'cup', 'unité', etc.
    - If English is missing, translate or null.
    - times: in minutes.
    JSON: {
      "title_fr": "", "title_en": "", "description_fr": "", "description_en": "",
      "servings": 1, "prep_time": 10, "cook_time": 20,
      "ingredients": [{"name_fr": "", "name_en": "", "quantity": 1.5, "unit_symbol": "g", "note_fr": ""}],
      "steps_fr": [""], "steps_en": [""]
    }"#;

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&json!({
            "model": "llama3.2:3b",
            "prompt": format!("{}\n\nOCR:\n{}", system_instructions, cleaned_text),
            "stream": false,
            "format": "json",
            "options": { "temperature": 0 }
        }))
        .send()
        .await
        .map_err(|_| Error::InternalServerError)?;
    println!("{:#?}", response);

    let res_body: serde_json::Value = response.json().await.map_err(|_| Error::InternalServerError)?;
    let structured_str = res_body["response"].as_str().unwrap_or("{}");
    let raw: SimpleExtraction = serde_json::from_str(structured_str).map_err(|_| Error::InternalServerError)?;

    // 1. Map Main Translations
    let translations = vec![
        RecipeTranslationInput {
            language_code: "fr".to_string(),
            title: raw.title_fr,
            description: raw.description_fr.unwrap_or_default(),
        },
        RecipeTranslationInput {
            language_code: "en".to_string(),
            title: raw.title_en.unwrap_or_else(|| "Untitled".to_string()),
            description: raw.description_en.unwrap_or_default(),
        },
    ];

    // 2. Map Ingredients with Decimal and UUID lookups
    let ingredients = raw.ingredients.into_iter().enumerate().map(|(i, ing)| {
        let unit_id = unit_map.get(&ing.unit_symbol.to_lowercase()).cloned().unwrap_or(Uuid::nil());
        IngredientInput {
            unit_id,
            // Convert f64 to Decimal for your DTO
            quantity: Decimal::from_f64_retain(ing.quantity).unwrap_or(Decimal::ZERO),
            position: i as i32,
            translations: vec![
                IngredientTranslationInput {
                    language_code: "fr".to_string(),
                    data: ing.name_fr,
                    note: ing.note_fr,
                },
                IngredientTranslationInput {
                    language_code: "en".to_string(),
                    data: ing.name_en.unwrap_or_default(),
                    note: None,
                }
            ],
        }
    }).collect();

    let ingredient_groups = vec![IngredientGroupInput {
        position: 0,
        translations: vec![IngredientGroupTranslationInput {
            language_code: "fr".to_string(),
            title: "Ingrédients".to_string()
        }],
        ingredients,
    }];

    // 3. Map Steps
    let steps = raw.steps_fr.into_iter().enumerate().map(|(i, s_fr)| {
        StepInput {
            position: i as i32,
            image_url: None,
            duration_minutes: None,
            translations: vec![
                StepTranslationInput { language_code: "fr".to_string(), instruction: s_fr },
                StepTranslationInput {
                    language_code: "en".to_string(),
                    instruction: raw.steps_en.as_ref().and_then(|v| v.get(i)).cloned().unwrap_or_default()
                },
            ],
        }
    }).collect();

    let step_groups = vec![StepGroupInput {
        position: 0,
        translations: vec![StepGroupTranslationInput {
            language_code: "fr".to_string(),
            title: "Instructions".to_string()
        }],
        steps,
    }];

    Ok(CreateRecipeInput {
        primary_language: "fr".to_string(),
        translations,
        servings: raw.servings,
        prep_time_minutes: raw.prep_time,
        cook_time_minutes: raw.cook_time,
        author_id: None,
        author: None,
        is_private: false,
        image_url: "/assets/recipes/default.png".to_string(),
        tags: vec![],
        ingredient_groups,
        step_groups,
    })
}

pub fn clean_ocr(text: &str) -> String {
    let re_garbage = Regex::new(r"[^a-zA-Z0-9\sÀ-ÿ,.!?:'/-]").unwrap();
    let re_newlines = Regex::new(r"\n{2,}]").unwrap();
    let cleaned = re_garbage.replace_all(text, " ");
    re_newlines.replace_all(&cleaned, "\n").to_string()
}
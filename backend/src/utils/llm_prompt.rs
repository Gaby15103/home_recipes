use crate::dto::recipe_dto::CreateRecipeInput;
use crate::dto::unit_dto::UnitDto;
use crate::errors::Error;
use serde_derive::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}
pub async fn process_ocr_to_dto(
    ocr_text: &str,
    units: Vec<UnitDto>,
) -> Result<CreateRecipeInput, Error> {
    let client = reqwest::Client::new();

    let mut unit_map: HashMap<String, Uuid> = HashMap::new();
    for u in &units {
        unit_map.insert(u.symbol.to_lowercase(), u.id);
        unit_map.insert(u.name_fr.to_lowercase(), u.id);
        unit_map.insert(u.name_en.to_lowercase(), u.id);
        unit_map.insert(u.code.to_lowercase(), u.id);
    }

    let units_reference = units
        .iter()
        .map(|u| format!("- {} ({}): use symbol '{}'", u.name_fr, u.name_en, u.symbol))
        .collect::<Vec<String>>()
        .join("\n");

    let system_instructions = format!(
        r#"Extract recipe to JSON.
        Rules:
        1. 'quantity' MUST be a number or string number (e.g. "1.5").
        2. 'unit_id' MUST be a symbol from the list below.
        3. Priority: "250 ml (1 t)" -> quantity: 1, unit_id: "cup".
        4. "1 oignon" -> quantity: 1, unit_id: "unité".
        5. Structure: Match the template. Always include 'fr' and 'en' translations.

        UNITS:
        {}

        TEMPLATE:
        {{
          "primary_language": "fr",
          "translations": [{{ "language_code": "fr", "title": "", "description": "" }}],
          "servings": 1,
          "ingredient_groups": [{{
            "translations": [{{ "language_code": "fr", "title": "Ingrédients" }}],
            "ingredients": [{{
              "translations": [{{ "language_code": "fr", "data": "Nom", "note": "prep" }}],
              "quantity": 1,
              "unit_id": "symbol"
            }}]
          }}],
          "step_groups": [{{
            "translations": [{{ "language_code": "fr", "title": "Instructions" }}],
            "steps": [{{ "translations": [{{ "language_code": "fr", "instruction": "" }}] }}]
          }}]
        }}"#,
        units_reference
    );

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "mistral-nemo",
            "prompt": format!("{}\n\nOCR:\n{}", system_instructions, ocr_text),
            "stream": false,
            "format": "json",
            "options": { "temperature": 0 }
        }))
        .send()
        .await
        .map_err(|_| Error::InternalServerError)?;

    let res_body: serde_json::Value = response.json().await.unwrap();
    let structured_str = res_body["response"].as_str().unwrap_or("{}");
    let mut val: serde_json::Value = serde_json::from_str(structured_str).unwrap_or(json!({}));

    // --- ROBUST REPAIR & HYDRATION ---

    // 1. Root level defaults
    if val.get("image_url").is_none() { val["image_url"] = json!("/assets/recipes/default.png"); }
    if val.get("is_private").is_none() { val["is_private"] = json!(false); }
    if val.get("tags").is_none() { val["tags"] = json!([]); }

    // 2. Repair Ingredient Groups & Ingredients
    if let Some(groups) = val.get_mut("ingredient_groups").and_then(|g| g.as_array_mut()) {
        for (g_idx, group) in groups.iter_mut().enumerate() {
            group["position"] = json!(g_idx);

            if let Some(ingredients) = group.get_mut("ingredients").and_then(|i| i.as_array_mut()) {
                for (i_idx, ing) in ingredients.iter_mut().enumerate() {
                    ing["position"] = json!(i_idx);

                    // Fix Quantity: Handle "1", 1, or null
                    let qty = match &ing["quantity"] {
                        serde_json::Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
                        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
                        _ => 0.0
                    };
                    ing["quantity"] = json!(qty);

                    // Fix Unit: Map Symbol/Name to UUID
                    let unit_str = ing["unit_id"].as_str().unwrap_or("unité").to_lowercase();
                    let uuid = unit_map.get(&unit_str).cloned().unwrap_or(Uuid::nil());
                    ing["unit_id"] = json!(uuid);

                    // Fix Ingredient Translations
                    if let Some(trans) = ing.get_mut("translations").and_then(|t| t.as_array_mut()) {
                        for t in trans {
                            if t.get("position").is_none() { t["position"] = json!(0); }
                            if t.get("note").is_none() { t["note"] = json!(null); }
                        }
                    }
                }
            }
        }
    }

    // 3. Repair Step Groups & Steps
    if let Some(step_groups) = val.get_mut("step_groups").and_then(|g| g.as_array_mut()) {
        for (sg_idx, s_group) in step_groups.iter_mut().enumerate() {
            s_group["position"] = json!(sg_idx);

            if let Some(steps) = s_group.get_mut("steps").and_then(|s| s.as_array_mut()) {
                for (s_idx, step) in steps.iter_mut().enumerate() {
                    step["position"] = json!(s_idx);
                    if step.get("image_url").is_none() { step["image_url"] = json!(null); }

                    if let Some(trans) = step.get_mut("translations").and_then(|t| t.as_array_mut()) {
                        for t in trans {
                            if t.get("position").is_none() { t["position"] = json!(0); }
                        }
                    }
                }
            }
        }
    } else {
        val["step_groups"] = json!([]);
    }

    // 4. Final Final mapping
    let dto: CreateRecipeInput = serde_json::from_value(val).map_err(|e| {
        eprintln!("FINAL MAPPING ERROR: {}", e);
        Error::InternalServerError
    })?;

    Ok(dto)
}

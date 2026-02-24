use crate::dto::recipe_dto::CreateRecipeInput;
use crate::errors::Error;
use serde_derive::Deserialize;
use serde_json::json;
use crate::dto::unit_dto::UnitDto;

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}
pub async fn process_ocr_to_dto(ocr_text: &str, units:Vec<UnitDto>) -> Result<CreateRecipeInput, Error> {
    let client = reqwest::Client::new();

    let units_reference = serde_json::to_string_pretty(&units).unwrap_or_else(|_| "[]".to_string());

    // 1. THIS IS THE "WHERE" AND "WHEN": Define the specific task
    let system_instructions = format!(r#"
        Task: Extract recipe data from OCR text into a specific JSON structure.

        ### UNIT REFERENCE (Use these UUIDs for unit_id):
        {}

        Rules:
        1. 'translations' must contain arrays for French (fr) and English (en).
        2. 'ingredient_groups' -> 'ingredients' -> 'translations' must contain 'data' (the name) and 'note'.
        3. 'quantity' must be a number (Decimal).
        4. 'unit_id' must be a UUID. Use '00000000-0000-0000-0000-000000000000' if unknown.
        5. Return ONLY valid JSON.

        CRITICAL RULES:
        1. 'step_groups' is a TOP-LEVEL array. It is NOT inside 'ingredient_groups'.
        2. Every object in 'step_groups' MUST have 'position', 'translations', and 'steps'.
        3. Even if there are no steps, return "step_groups": [].
        4. Follow the DTO structure exactly or the system will crash.

        JSON Structure Template:
        {{
          "primary_language": "fr",
          "translations": [
            {{ "language_code": "fr", "title": "...", "description": "" }},
            {{ "language_code": "en", "title": "...", "description": "" }}
          ],
          "image_url": "",
          "servings": 4,
          "prep_time_minutes": 10,
          "cook_time_minutes": 20,
          "is_private": false,
          "tags": [],
          "ingredient_groups": [{{
            "position": 0,
            "translations": [{{ "language_code": "fr", "title": "Ingrédients" }}],
            "ingredients": [{{
              "translations": [{{ "language_code": "fr", "data": "Farine", "note": "tamisée" }}],
              "quantity": 500.0,
              "unit_id": "UUID_HERE",
              "position": 0
            }}]
          }}],
          "step_groups": [{{
            "position": 0,
            "translations": [{{ "language_code": "fr", "title": "Instructions" }}],
            "steps": [{{
              "position": 0,
              "translations": [{{ "language_code": "fr", "instruction": "..." }}]
            }}]
          }}]
        }}
    "#, units_reference);

    // 2. Combine instructions with the data
    let full_prompt = format!(
        "{}\n\n### OCR TEXT TO PROCESS:\n{}",
        system_instructions, ocr_text
    );

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "llama3",
            "prompt": full_prompt,
            "stream": false,
            "format": "json" // This locks Ollama into JSON mode
        }))
        .send()
        .await
        .map_err(|_| Error::InternalServerError)?;

    let res_json: serde_json::Value = response.json().await.unwrap();
    let mut structured_str = res_json["response"].as_str().unwrap_or("{}").to_string();

    // Sanitation: Remove Markdown code blocks if the AI included them
    if structured_str.contains("```json") {
        structured_str = structured_str
            .replace("```json", "")
            .replace("```", "")
            .trim()
            .to_string();
    }

    // Pre-parse check for mandatory top-level fields
    let mut val: serde_json::Value = serde_json::from_str(&structured_str).map_err(|_| Error::InternalServerError)?;

    println!("{}", val);

    // If the AI missed step_groups, inject an empty array so Serde doesn't crash
    if val.get("step_groups").is_none() {
        val["step_groups"] = serde_json::json!([]);
    }

    let dto: CreateRecipeInput = serde_json::from_value(val)
        .map_err(|e| {
            eprintln!("SERDE ERROR: {}", e);
            Error::BadRequest(serde_json::json!({ "error": "Mapping failed", "details": e.to_string() }))
        })?;

    Ok(dto)
}

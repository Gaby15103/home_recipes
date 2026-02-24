use serde_derive::Deserialize;
use serde_json::json;
use crate::dto::recipe_dto::CreateRecipeInput;
use crate::errors::Error;
#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}
pub async fn process_ocr_to_dto(ocr_text: &str) -> Result<CreateRecipeInput, Error> {
    let client = reqwest::Client::new();

    // 1. THIS IS THE "WHERE" AND "WHEN": Define the specific task
    let system_instructions = r#"
        Task: Extract recipe data into JSON.
        Format: Return ONLY a JSON object matching this DTO:
        {
          "primary_language": "fr",
          "translations": [{"language": "fr", "title": "...", "description": ""}, {"language": "en", "title": "...", "description": ""}],
          "servings": 0,
          "prep_time_minutes": 0,
          "cook_time_minutes": 0,
          "ingredient_groups": [{"name": "Ingredients", "ingredients": [{"name": "...", "quantity": 0.0, "unit": "..."}]}],
          "step_groups": [{"name": "Instructions", "steps": [{"instruction": "..."}]}]
        }
        Notes: Translate the 'en' translation title from the French text. If units are 'ml' or 'g', keep them.
    "#;

    // 2. Combine instructions with the data
    let full_prompt = format!("{}\n\n### OCR TEXT TO PROCESS:\n{}", system_instructions, ocr_text);

    let response = client.post("http://localhost:11434/api/generate")
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
    println!("{}", res_json);
    let structured_str = res_json["response"].as_str().unwrap_or("{}");
    println!("{}", structured_str);
    // 3. Convert to your Rust Struct
    let dto: CreateRecipeInput = serde_json::from_str(structured_str)
        .map_err(|e| {
            // This is your best friend right now:
            eprintln!("SERDE ERROR: {}", e);
            eprintln!("ON COLUMN: {}", e.column());

            Error::BadRequest(serde_json::json!({
            "error": "AI parsing failed",
            "details": e.to_string()
        }))
        })?;

    Ok(dto)
}
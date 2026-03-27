use crate::dto::recipe_dto::{CreateRecipeInput, RecipeDto, RecipeViewDto};
use crate::dto::recipe_ocr::{OcrConfirmInput, OcrCorrectionWrapper, OcrResultResponse}; // Returning the bridge DTO instead
use crate::errors::Error;
use crate::recipe_parser;
use crate::recipe_parser::{ParserContext, teach_lexicon};
use crate::repositories::unit_repository;
use crate::services::recipe_service;
use actix_multipart::form::tempfile::TempFile;
use sea_orm::DatabaseConnection;
use serde_json::json;
use sqlx::SqlitePool;
use base64::{Engine as _, engine::general_purpose};
use crate::dto::upload_dto::RegionDto;
use crate::utils::schema::get_cleaned_gemini_schema;

pub async fn recipe_from_files(
    images: Vec<TempFile>,
    db: &DatabaseConnection,
    gemini_api_key: &str,
) -> Result<CreateRecipeInput, Error> {
    let units = unit_repository::get_all_admin(db).await?;
    let schema_object = get_cleaned_gemini_schema();

    let unit_hints: String = units.iter()
        .map(|u| format!("{}: {}", u.symbol, u.id))
        .collect::<Vec<_>>()
        .join(", ");

    let mut parts = Vec::new();
    parts.push(json!({
        "text": format!(
            "Extract this recipe into the provided JSON schema. \
            Unit mapping (symbol: UUID): {}. \
            If a unit symbol isn't found, use '00000000-0000-0000-0000-000000000000'. \
            Provide translations for both 'fr' and 'en'.",
            unit_hints
        )
    }));

    for img in images {
        let bytes = std::fs::read(img.file.path()).map_err(|e| {
            log::error!("File read error: {}", e);
            Error::BadRequest("Could not read uploaded image".into())
        })?;
        parts.push(json!({
            "inline_data": {
                "mime_type": "image/jpeg",
                "data": general_purpose::STANDARD.encode(bytes)
            }
        }));
    }

    let client = reqwest::Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}", gemini_api_key);

    let mut attempts = 0;
    let res_json = loop {
        let response = client
            .post(&url)
            .json(&json!({
                "contents": [{ "parts": parts }],
                "generationConfig": {
                    "response_mime_type": "application/json",
                    "response_schema": schema_object,
                    "temperature": 0.1
                }
            }))
            .send()
            .await.map_err(|_| Error::InternalServerError)?;

        let status = response.status();

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            attempts += 1;
            if attempts >= 3 {
                return Err(Error::BadRequest("Gemini Quota exhausted. Try again in a minute.".into()));
            }
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            continue;
        }

        if !status.is_success() {
            let err_body: serde_json::Value = response.json().await.map_err(|_| Error::InternalServerError)?;
            let msg = err_body["error"]["message"].as_str().unwrap_or("Unknown Gemini Error");
            log::error!("Gemini API Error ({}): {}", status, msg);
            // Return the ACTUAL error message from Google to the user
            return Err(Error::BadRequest(format!("Gemini Error: {}", msg).into()));
        }

        break response.json::<serde_json::Value>().await.map_err(|_| Error::InternalServerError)?;
    };

    // Check for Safety Blocks (FinishReason)
    let candidate = res_json["candidates"].get(0).ok_or_else(|| {
        let block_reason = res_json["promptFeedback"]["blockReason"].as_str().unwrap_or("Unknown");
        Error::BadRequest(format!("Content blocked by Google Safety: {}", block_reason).into())
    })?;

    if let Some(reason) = candidate["finishReason"].as_str() {
        if reason == "SAFETY" || reason == "OTHER" {
            return Err(Error::BadRequest(format!("Gemini failed to finish: {}", reason).into()));
        }
    }

    let json_text = candidate["content"]["parts"]
        .get(0)
        .and_then(|part| part["text"].as_str())
        .ok_or_else(|| Error::BadRequest("Gemini returned no text content".into()))?;

    let clean_json = json_text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let recipe_input: CreateRecipeInput = serde_json::from_str(clean_json).map_err(|e| {
        log::error!("JSON Parse Error: {}. Raw text: {}", e, clean_json);
        Error::BadRequest(format!("Failed to parse JSON into Recipe: {}", e).into())
    })?;

    Ok(recipe_input)
}
pub async fn recipe_from_regions(
    images: Vec<TempFile>,
    regions: Vec<RegionDto>,
    lang: String,
    db: &DatabaseConnection,
    sqlite_pool: &SqlitePool,
) -> Result<OcrResultResponse, Error> {
    let units = unit_repository::get_all_admin(db).await?;
    let context = ParserContext { sqlite_pool, known_units: units };

    // Just pass the raw images and regions into the parser
    recipe_parser::run_region_pipeline(images, regions, &lang, context).await
}
pub async fn process_ocr_confirmation(
    payload: OcrCorrectionWrapper,
    pg_db: &DatabaseConnection,
    sqlite_pool: &SqlitePool,
    preferred_language: &str,
) -> Result<RecipeViewDto, Error> {
    // 1. TEACH: Compare original strings to the user's final selections
    teach_lexicon(&payload, sqlite_pool).await?;

    // 2. CONVERT: Use the internal method of the modified part of the payload
    let create_input = payload.modified_recipe.to_create_input();

    // 3. PERSIST: Save the clean recipe to Postgres
    let result = recipe_service::create(pg_db, create_input, preferred_language).await?;

    Ok(result)
}

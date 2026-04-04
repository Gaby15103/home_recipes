use crate::dto::recipe_dto::{CreateRecipeInput, RecipeDto, RecipeViewDto};
use crate::dto::recipe_ocr::{OcrConfirmInput, OcrCorrectionWrapper, OcrResultResponse}; // Returning the bridge DTO instead
use crate::dto::upload_dto::RegionDto;
use crate::errors::Error;
use crate::recipe_parser;
use crate::recipe_parser::{ParserContext, teach_lexicon};
use crate::repositories::unit_repository;
use crate::services::recipe_service;
use crate::utils::schema::get_cleaned_gemini_schema;
use actix_multipart::form::tempfile::TempFile;
use base64::{Engine as _, engine::general_purpose};
use sea_orm::DatabaseConnection;
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub async fn recipe_from_files(
    images: Vec<TempFile>,
    db: &DatabaseConnection,
    open_router_key: &str, // Changed name for clarity
) -> Result<CreateRecipeInput, Error> {
    // 1. Generate a Hash for Cache Check (Keep as is)
    let mut hasher = Sha256::new();
    let mut image_data = Vec::new();
    for img in &images {
        let bytes = fs::read(img.file.path())
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to read image file",
                "operation": "recipe_from_files",
                "file_path": img.file.path().to_string_lossy(),
                "error": e.to_string(),
                "stage": "file_read"
            })))?;
        hasher.update(&bytes);
        image_data.push(bytes);
    }
    let hash_str = hex::encode(hasher.finalize());
    let cache_dir = Path::new("api_cache");
    let cache_path = cache_dir.join(format!("{}.json", hash_str));

    // 2. Try to Load from Cache (Keep as is)
    if cache_path.exists() {
        let cached_json = fs::read_to_string(&cache_path)
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to read cache file",
                "operation": "recipe_from_files",
                "cache_path": cache_path.to_string_lossy(),
                "error": e.to_string(),
                "stage": "cache_read"
            })))?;

        return serde_json::from_str(&cached_json)
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to parse cached recipe JSON",
                "operation": "recipe_from_files",
                "error": e.to_string(),
                "stage": "cache_deserialization"
            })));
    }

    // 3. Prepare OpenRouter Payload
    let mut content_parts = Vec::new();
    content_parts.push(json!({
        "type": "text",
        "text": "Extract this recipe into JSON. For 'unit_id', use the unit symbol (e.g. 'g', 'ml', 'cup'). Map 'fr' and 'en' translations."
    }));

    for bytes in image_data {
        let b64 = general_purpose::STANDARD.encode(bytes);
        content_parts.push(json!({
            "type": "image_url",
            "image_url": { "url": format!("data:image/jpeg;base64,{}", b64) }
        }));
    }

    content_parts.push(json!({
        "type": "text",
        "text": "Extract recipe to JSON.
             IMPORTANT RULES:
             1. 'amount' MUST be a NUMBER, not a string (e.g., 6, not '6').
             2. 'unit_id' MUST be the unit SYMBOL string (e.g., 'g').
             3. Omit null fields."
    }));

    // 4. Call OpenRouter API
    let client = reqwest::Client::new();
    let models = vec![
        "openrouter/free",
        "google/gemma-3-12b:free",
        "google/gemma-3-4b:free",
        "nvidia/llama-nemotron-embed-vl-1b-v2:free",
    ];

    let mut attempts = 0;

    let json_text = loop {
        let current_model = models[attempts % models.len()];

        let res: serde_json::Value = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", open_router_key))
            .header("HTTP-Referer", "http://localhost:5173")
            .header("X-Title", "HomeRecipes")
            .json(&json!({
                "model": current_model,
                "messages": [{ "role": "user", "content": content_parts }],
                "response_format": { "type": "json_object" },
                "temperature": 0.1
            }))
            .send()
            .await
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to send request to OpenRouter API",
                "operation": "recipe_from_files",
                "model": current_model,
                "attempt": attempts,
                "error": e.to_string(),
                "stage": "api_request"
            })))?
            .json()
            .await
            .map_err(|e| Error::InternalServerError(json!({
                "message": "Failed to parse OpenRouter API response",
                "operation": "recipe_from_files",
                "model": current_model,
                "attempt": attempts,
                "error": e.to_string(),
                "stage": "api_response_parse"
            })))?;

        println!("{:#?}", res);

        // Check for success
        if let Some(content) = res["choices"][0]["message"]["content"].as_str() {
            if !content.is_empty() {
                break content.to_string();
            }
        }

        // If we get here, it failed or was empty
        attempts += 1;
        if attempts >= models.len() {
            log::error!("All free models failed. Last response: {:?}", res);
            return Err(Error::BadRequest(json!({
                "error": "All providers failed to return content. Try again later."
            })));
        }

        log::warn!("Model {} failed, trying {}...", current_model, models[attempts % models.len()]);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    };

    // 6. Post-Process (Mapping symbols to UUIDs)
    let units = unit_repository::get_all_admin(db).await?;
    let mut recipe_input: CreateRecipeInput = serde_json::from_str(&*json_text)
        .map_err(|e| Error::BadRequest(format!("Schema mismatch: {}", e).into()))?;

    for group in &mut recipe_input.ingredient_groups {
        for ing in &mut group.ingredients {
            let matched_unit = units
                .iter()
                .find(|u| u.symbol.to_lowercase() == ing.unit_id.to_string().to_lowercase());

            ing.unit_id = match matched_unit {
                Some(u) => u.id,
                None => Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
            };
        }
    }

    // 7. Save to Cache
    fs::create_dir_all(cache_dir).ok();
    fs::write(&cache_path, serde_json::to_string(&recipe_input)
        .map_err(|e| Error::InternalServerError(json!({
            "message": "Failed to serialize recipe for cache",
            "operation": "recipe_from_files",
            "cache_path": cache_path.to_string_lossy(),
            "error": e.to_string(),
            "stage": "cache_serialization"
        })))?)
        .ok();

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
    let context = ParserContext {
        sqlite_pool,
        known_units: units,
    };

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

    // 3. PERSIST: Save the clean recipe to Postgres
    let result = recipe_service::create(pg_db, payload.modified_recipe, preferred_language).await?;

    Ok(result)
}

use std::fs;
use std::path::Path;
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
use sha2::{Digest, Sha256};
use uuid::Uuid;
use crate::dto::upload_dto::RegionDto;
use crate::utils::schema::get_cleaned_gemini_schema;

pub async fn recipe_from_files(
    images: Vec<TempFile>,
    db: &DatabaseConnection,
    gemini_api_key: &str,
) -> Result<CreateRecipeInput, Error> {
    // 1. Generate a Hash for Cache Check
    let mut hasher = Sha256::new();
    let mut image_data = Vec::new();

    for img in &images {
        let bytes = fs::read(img.file.path()).map_err(|_| Error::InternalServerError)?;
        hasher.update(&bytes);
        image_data.push(bytes);
    }
    let hash_str = hex::encode(hasher.finalize());
    let cache_dir = Path::new("api_cache");
    let cache_path = cache_dir.join(format!("{}.json", hash_str));

    // 2. Try to Load from Cache
    if cache_path.exists() {
        log::info!("Cache hit for image hash: {}", hash_str);
        let cached_json = fs::read_to_string(&cache_path).map_err(|_| Error::InternalServerError)?;
        return Ok(serde_json::from_str(&cached_json)?);
    }

    // 3. Prepare Prompt (No UUIDs!)
    let mut parts = Vec::new();
    parts.push(json!({
        "text": "Extract this recipe. For 'unit_id', just write the unit symbol found in the text (e.g., 'g', 'ml', 'tsp', 'cup', 'unit'). I will map these to IDs later. Provide 'fr' and 'en' translations."
    }));

    for bytes in image_data {
        parts.push(json!({
            "inline_data": {
                "mime_type": "image/jpeg",
                "data": general_purpose::STANDARD.encode(bytes)
            }
        }));
    }

    // 4. Call Gemini API
    let client = reqwest::Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}", gemini_api_key);
    let schema_object = get_cleaned_gemini_schema();

    let res_json: serde_json::Value = client
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
        .await
        .map_err(|_| Error::InternalServerError)?
        .json()
        .await
        .map_err(|_| Error::InternalServerError)?;

    println!("{:#?}", res_json);

    // 5. Extract and Clean JSON
    let json_text = res_json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| Error::BadRequest("Gemini returned no content".into()))?;

    // 6. Post-Process: Map Symbols back to Database UUIDs
    let units = unit_repository::get_all_admin(db).await?;
    let mut recipe_input: CreateRecipeInput = serde_json::from_str(json_text).map_err(|e| {
        Error::BadRequest(format!("Schema mismatch: {}", e).into())
    })?;

    // Reassemble UUIDs based on symbols Gemini returned
    for group in &mut recipe_input.ingredient_groups {
        for ing in &mut group.ingredients {
            // Find the ID in our DB that matches the symbol Gemini gave us
            let matched_unit = units.iter().find(|u| {
                u.symbol.to_lowercase() == ing.unit_id.to_string().to_lowercase()
            });

            ing.unit_id = match matched_unit {
                Some(u) => u.id,
                None => Uuid::parse_str("00000000-0000-0000-0000-000000000000")?,
            };
        }
    }

    // 7. Save to Cache and Return
    fs::create_dir_all(cache_dir).ok();
    let final_json = serde_json::to_string(&recipe_input).map_err(|_| Error::InternalServerError)?;
    fs::write(cache_path, &final_json).ok();

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

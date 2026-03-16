use crate::dto::recipe_dto::{CreateRecipeInput, RecipeDto, RecipeViewDto};
use crate::dto::recipe_ocr::{OcrConfirmInput, OcrCorrectionWrapper, OcrResultResponse}; // Returning the bridge DTO instead
use crate::errors::Error;
use crate::recipe_parser;
use crate::recipe_parser::{ParserContext, teach_lexicon};
use crate::repositories::unit_repository;
use crate::services::recipe_service;
use actix_multipart::form::tempfile::TempFile;
use sea_orm::DatabaseConnection;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::dto::upload_dto::RegionDto;

pub async fn recipe_from_files(
    images: Vec<TempFile>,
    regions: Vec<RegionDto>,
    lang: String,
    db: &DatabaseConnection,
    sqlite_pool: &SqlitePool,
) -> Result<OcrResultResponse, Error> {
    // 1. Get units to help the parser resolve UUIDs early if possible
    let units = unit_repository::get_all_admin(db).await?;

    let context = ParserContext {
        sqlite_pool,
        known_units: units,
    };

    // 2. Extract paths for the multi-image scanner
    let paths: Vec<&std::path::Path> = images.iter().map(|f| f.file.path()).collect();

    // 3. Run the pipeline (Scan -> Classify -> Match)
    // This returns the structured suggestion with confidence levels
    let ocr_suggestions = recipe_parser::run_pipeline(&paths, context).await?;

    Ok(ocr_suggestions)
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

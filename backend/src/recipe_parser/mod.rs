use std::time::Instant;
use std::path::Path;
use crate::errors::Error;
use crate::dto::recipe_dto::CreateRecipeInput;
use crate::dto::unit_dto::UnitDto;
use sqlx::SqlitePool;

pub mod dictionary;
pub mod grammar;
pub mod scanner;

/// A clean way to pass dependencies into the parser engine
pub struct ParserContext<'a> {
    pub sqlite_pool: &'a SqlitePool,
    pub known_units: Vec<UnitDto>,
}

pub async fn run_pipeline(
    path: &Path,
    ctx: ParserContext<'_>
) -> Result<CreateRecipeInput, Error> {
    let total_start = Instant::now();

    // 1. Scanner: Image -> Raw String
    let scan_start = Instant::now();
    let raw_text = scanner::scan_image(path)?;
    let scan_duration = scan_start.elapsed();

    // 2. Dictionary: Raw String -> Smart Tokens (WordType)
    // We use the 'tokenize_text' function we built earlier
    let dict_start = Instant::now();
    let tokens = dictionary::tokenize_text(&raw_text, ctx.sqlite_pool).await?;
    let dict_duration = dict_start.elapsed();

    // 3. Grammar: Smart Tokens -> Final DTO
    let grammar_start = Instant::now();
    let recipe = grammar::map_to_dto(tokens, &ctx.known_units);
    let grammar_duration = grammar_start.elapsed();

    // --- 🛠️ Feedback Output ---
    println!("--- 🛠️ Recipe Parser Feedback ---");
    println!("📸 OCR Scan:      {:.2?}", scan_duration);
    println!("📖 Dict Cleanup:  {:.2?}", dict_duration);
    println!("🏗️ DTO Mapping:   {:.2?}", grammar_duration);
    println!("🚀 Total Time:    {:.2?}", total_start.elapsed());
    println!("---------------------------------");

    Ok(recipe)
}
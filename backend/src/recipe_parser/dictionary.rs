use sqlx::{Row, SqlitePool};
use crate::errors::Error;
use std::time::Instant;

/// Represents the categorized meaning of a word found via OCR.
#[derive(Debug, Clone)]
pub enum WordType {
    /// A recognized ingredient name (e.g., "flour", "sugar")
    Ingredient(String),
    /// A recognized measurement unit (e.g., "grams", "ml", "tbsp")
    Unit(String),
    /// A numerical value for quantity (e.g., 200, 1.5)
    Quantity(f32),
    /// Standard text that didn't match a specific category
    Text(String),
}

/// Tokenizes a raw OCR string into a sequence of categorized WordTypes.
/// This is the main entry point for the dictionary cleanup phase.
pub async fn tokenize_text(raw_text: &str, pool: &SqlitePool) -> Result<Vec<WordType>, Error> {
    let start = Instant::now();
    let mut tokens = Vec::new();

    // Split by whitespace to process word-by-word
    for word in raw_text.split_whitespace() {
        // Remove punctuation like commas or dots for the lookup (e.g., "flour," -> "flour")
        let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != ',');

        if !clean_word.is_empty() {
            let token = lookup_word(clean_word, pool).await?;
            tokens.push(token);
        }
    }

    println!(
        "⏱️ [Dictionary] Tokenized {} words into {} tokens in {:.2?}",
        raw_text.split_whitespace().count(),
        tokens.len(),
        start.elapsed()
    );

    Ok(tokens)
}

/// Queries the SQLite database to find the canonical version of a word.
async fn lookup_word(word: &str, pool: &SqlitePool) -> Result<WordType, Error> {
    // 1. Check if the word is a number (Quantity)
    if let Ok(num) = word.replace(',', ".").parse::<f32>() {
        return Ok(WordType::Quantity(num));
    }

    // 2. Query the SQLite Lexicon and Aliases
    let result = sqlx::query(
        r#"
        SELECT l.term_en, l.category
        FROM lexicon l
        LEFT JOIN aliases a ON a.lexicon_id = l.id
        WHERE a.raw_text = ? OR l.term_en = ? OR l.term_fr = ?
        COLLATE NOCASE
        LIMIT 1
        "#
    )
        .bind(word)
        .bind(word)
        .bind(word)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Database Error: {:?}", e);
            Error::InternalServerError
        })?;

    // 3. Map the database row to the WordType enum
    match result {
        Some(row) => {
            // Manually extract columns by name or index
            let term_en: String = row.get("term_en");
            let category: String = row.get("category");

            match category.to_lowercase().as_str() {
                "ingredient" => Ok(WordType::Ingredient(term_en)),
                "unit"       => Ok(WordType::Unit(term_en)),
                _            => Ok(WordType::Text(term_en)),
            }
        },
        // If not in DB, keep it as raw text
        None => Ok(WordType::Text(word.to_string())),
    }
}
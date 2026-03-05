use sqlx::{Row, SqlitePool};
use crate::dto::recipe_ocr::OcrMatchMetadata;
use crate::errors::Error;

pub async fn resolve_line(
    line: &str,
    pool: &SqlitePool
) -> Result<(Option<f32>, Option<OcrMatchMetadata>, Option<OcrMatchMetadata>, Vec<OcrMatchMetadata>), Error> {
    let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

    let mut quantity = None;
    let mut unit = None;
    let mut ingredient = None;
    let mut actions = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        let mut matched = false;

        // 1. TRY THE SLIDING WINDOW (Longest match first: 3 words down to 1)
        for size in (1..=3).rev() {
            if i + size <= tokens.len() {
                let chunk = tokens[i..i+size].join(" ");

                if let Some(metadata) = lookup_lexicon(&chunk, pool).await? {
                    match metadata.category.as_str() {
                        "unit" => unit = Some(metadata),
                        "ingredient" => ingredient = Some(metadata),
                        "action" => actions.push(metadata),
                        _ => {}
                    }
                    i += size;
                    matched = true;
                    break;
                }
            }
        }

        // 2. FALLBACK: If no dictionary match, check if it's a number
        if !matched {
            if quantity.is_none() {
                if let Ok(val) = parse_numeric(&tokens[i]) {
                    quantity = Some(val);
                }
            }
            i += 1;
        }
    }

    Ok((quantity, unit, ingredient, actions))
}

async fn lookup_lexicon(text: &str, pool: &SqlitePool) -> Result<Option<OcrMatchMetadata>, Error> {
    let word = text.to_lowercase().trim().to_string();

    // Standard sqlx::query does NOT require a DB connection at compile time
    let row = sqlx::query(
        r#"
        SELECT l.id, l.term_en, l.term_fr, l.category, a.confidence
        FROM lexicon l
        LEFT JOIN aliases a ON a.lexicon_id = l.id
        WHERE a.raw_text = ? OR l.term_en = ? OR l.term_fr = ?
        COLLATE NOCASE
        LIMIT 1
        "#
    )
        .bind(&word)
        .bind(&word)
        .bind(&word)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Lexicon Database Error: {:?}", e);
            Error::InternalServerError
        })?;

    // Manually map the row to your DTO
    if let Some(r) = row {
        Ok(Some(OcrMatchMetadata {
            raw_token: text.to_string(),
            lexicon_id: r.get::<i32, _>("id"),
            term_en: r.get::<String, _>("term_en"),
            term_fr: Option::from(r.get::<String, _>("term_fr")),
            category: r.get::<Option<String>, _>("category").unwrap_or_else(|| "ingredient".to_string()),
            confidence: r.get::<Option<f64>, _>("confidence").unwrap_or(1.0) as f32,
            match_strategy: "fuzzy_alias".to_string(),
        }))
    } else {
        Ok(None)
    }
}

fn parse_numeric(token: &str) -> Result<f32, ()> {
    // Handle Unicode fractions like ½ or ¾
    match token {
        "¼" => return Ok(0.25),
        "½" => return Ok(0.5),
        "¾" => return Ok(0.75),
        _ => {}
    }

    // Handle standard fractions like 1/2
    if token.contains('/') {
        let parts: Vec<&str> = token.split('/').collect();
        if parts.len() == 2 {
            if let (Ok(n), Ok(d)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                return Ok(n / d);
            }
        }
    }

    token.parse::<f32>().map_err(|_| ())
}
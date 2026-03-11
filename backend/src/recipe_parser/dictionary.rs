use sqlx::{Row, SqlitePool};
use crate::dto::recipe_ocr::OcrMatchMetadata;
use crate::errors::Error;

fn is_stop_word(word: &str) -> bool {
    let stops = ["de", "d'", "du", "des", "le", "la", "les", "au", "aux", "ou", "with", "and"];
    stops.contains(&word.to_lowercase().as_str())
}

pub async fn resolve_line(
    line: &str,
    pool: &SqlitePool
) -> Result<(Option<f32>, Option<OcrMatchMetadata>, Option<OcrMatchMetadata>, Vec<OcrMatchMetadata>), Error> {
    let re_artifact = regex::Regex::new(r"(?i)^[^a-z\d¼½¾⅓⅔⅛⅜⅝⅞]+").unwrap();
    let cleaned_line = re_artifact.replace(line, "").to_string();

    let tokens: Vec<String> = cleaned_line.split_whitespace().map(|s| s.to_string()).collect();

    let mut quantity = None;
    let mut unit = None;
    let mut ingredient = None;
    let mut actions = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        if is_stop_word(&tokens[i]) || (tokens[i].len() <= 1 && !tokens[i].chars().all(|c| c.is_numeric() || "¼½¾⅓⅔⅛⅜⅝⅞".contains(c))) {
            i += 1;
            continue;
        }

        let mut matched = false;

        // Multi-word greedy matching (e.g., "olive oil" or "pomme de terre")
        for size in (1..=3).rev() {
            if i + size <= tokens.len() {
                let chunk = tokens[i..i+size].join(" ");

                if let Some(metadata) = lookup_lexicon(&chunk, pool).await? {
                    match metadata.category.as_str() {
                        "unit" => unit = Some(metadata),
                        "ingredient" => ingredient = Some(metadata),
                        "action" | "descriptor" | "text" => actions.push(metadata),
                        _ => {}
                    }
                    i += size;
                    matched = true;
                    break;
                }
            }
        }

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
    // 1. Clean the input and handle empty strings immediately
    let word = text.to_lowercase().trim().to_string();

    // FTS5 will crash on empty queries or just punctuation
    let clean_query = word.replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), "");

    if clean_query.is_empty() {
        return Ok(None);
    }

    // FTS5 Match query. We use the rank to adjust confidence.
    // We join back to lexicon to get the category and standardized terms.
    let fts_query = format!("\"{}\"", clean_query);

    let row = sqlx::query(
        r#"
        SELECT
            l.id, l.term_en, l.term_fr, l.category, f.rank,
            a.confidence as base_confidence
        FROM lexicon_fts f
        JOIN lexicon l ON l.id = f.lexicon_id
        LEFT JOIN aliases a ON a.lexicon_id = l.id AND a.raw_text = f.raw_text
        WHERE lexicon_fts MATCH ?
        ORDER BY rank
        LIMIT 1
        "#
    )
        .bind(&fts_query)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("FTS Search Error for query [{}]: {:?}", fts_query, e);
            Error::InternalServerError
        })?;

    if let Some(r) = row {
        let rank: f64 = r.get("rank");
        let base_conf: f32 = r.get::<Option<f64>, _>("base_confidence").unwrap_or(1.0) as f32;

        let final_confidence = if rank > -0.5 { base_conf * 0.8 } else { base_conf };

        Ok(Some(OcrMatchMetadata {
            raw_token: text.to_string(),
            lexicon_id: r.get::<i32, _>("id"),
            term_en: r.get::<String, _>("term_en"),
            term_fr: r.get::<Option<String>, _>("term_fr"),
            category: r.get::<Option<String>, _>("category").unwrap_or_else(|| "ingredient".to_string()),
            confidence: final_confidence,
            match_strategy: "fts5_fuzzy".to_string(),
        }))
    } else {
        Ok(None)
    }
}

pub fn parse_numeric(token: &str) -> Result<f32, ()> {
    // Handle Unicode Vulgar Fractions
    let unicode_fractions = [
        ('¼', 0.25), ('½', 0.5), ('¾', 0.75), ('⅓', 0.333),
        ('⅔', 0.666), ('⅛', 0.125), ('⅜', 0.375), ('⅝', 0.625), ('⅞', 0.875)
    ];

    for (c, val) in unicode_fractions {
        if token.contains(c) {
            let clean = token.replace(c, "").trim().to_string();
            let base = if clean.is_empty() { 0.0 } else { clean.parse::<f32>().unwrap_or(0.0) };
            return Ok(base + val);
        }
    }

    // Handle standard ASCII fractions
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

/// Helper for planned improvement #4: extracting actions from instructions
pub async fn find_actions_in_text(text: &str, pool: &SqlitePool) -> Result<Vec<OcrMatchMetadata>, Error> {
    let tokens: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    let mut detected = Vec::new();

    for token in tokens {
        if let Some(meta) = lookup_lexicon(&token, pool).await? {
            if meta.category == "action" {
                detected.push(meta);
            }
        }
    }
    Ok(detected)
}
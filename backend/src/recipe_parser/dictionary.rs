use sqlx::{Row, SqlitePool};
use crate::dto::recipe_ocr::OcrMatchMetadata;
use crate::errors::Error;
use regex::Regex;

/// Checks if a word is a functional "noise" word that should never trigger a DB search.
fn is_stop_word(word: &str) -> bool {
    let stops = [
        "de", "d'", "du", "des", "le", "la", "les", "au", "aux",
        "ou", "en", "par", "pour", "with", "and", "d", "l", "un", "une",
        "atab", "c.", "thé", "tab", "xde", "x", "—"
    ];
    let clean = word.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string();
    stops.contains(&clean.as_str())
}

pub async fn resolve_line(
    line: &str,
    pool: &SqlitePool
) -> Result<(Option<f32>, Option<OcrMatchMetadata>, Option<OcrMatchMetadata>, Vec<OcrMatchMetadata>, String, String), Error> {
    // 1. Remove leading artifacts and noise
    let re_artifact = Regex::new(r"(?i)^[^a-z\d¼½¾⅓⅔⅛⅜⅝⅞]+").unwrap();
    let cleaned_line = re_artifact.replace(line, "").to_string();

    let tokens: Vec<String> = cleaned_line.split_whitespace().map(|s| s.to_string()).collect();

    let mut quantity = None;
    let mut unit = None;
    let mut ingredient = None;
    let mut actions = Vec::new();

    let mut display_indices = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        let current_token = &tokens[i];
        let has_letters = current_token.chars().any(|c| c.is_alphabetic());
        let is_fraction = current_token.chars().all(|c| "¼½¾⅓⅔⅛⅜⅝⅞/".contains(c) || c.is_numeric());

        let mut matched = false;

        // Multi-word greedy matching
        for size in (1..=5).rev() {
            if i + size <= tokens.len() {
                let chunk = tokens[i..i+size].join(" ");

                if is_stop_word(&chunk) {
                    continue;
                }

                if let Some(metadata) = lookup_lexicon(&chunk, pool).await? {
                    match metadata.category.as_str() {
                        "unit" => if unit.is_none() { unit = Some(metadata) },
                        "ingredient" => {
                            ingredient = Some(metadata.clone());
                            for j in 0..size { display_indices.push(i + j); }
                        },
                        "action" | "descriptor" | "text" => {
                            actions.push(metadata);
                            for j in 0..size { display_indices.push(i + j); }
                        },
                        _ => {}
                    }
                    i += size;
                    matched = true;
                    break;
                }
            }
        }

        if !matched {
            if quantity.is_none() && !has_letters && is_fraction {
                if let Ok(val) = parse_numeric(current_token) {
                    quantity = Some(val);
                }
            } else if !is_stop_word(current_token) && has_letters {
                display_indices.push(i);
            }
            i += 1;
        }
    }

    display_indices.sort_unstable();
    display_indices.dedup();

    // Create the base display name (usually in the analyzed language)
    let display_name = display_indices.iter()
        .map(|&idx| tokens[idx].as_str())
        .collect::<Vec<_>>()
        .join(" ");

    // Return the tuple with 6 elements to match the bilingual requirements in grammar.rs
    Ok((
        quantity,
        unit,
        ingredient,
        actions,
        display_name.clone(), // disp_en
        display_name          // disp_fr (will be overridden/translated in grammar.rs if needed)
    ))
}

async fn lookup_lexicon(text: &str, pool: &SqlitePool) -> Result<Option<OcrMatchMetadata>, Error> {
    let word = text.to_lowercase().trim().to_string();
    let clean_query = word.replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), "");

    if clean_query.is_empty() {
        return Ok(None);
    }

    let exact_query = format!("\"{}\"", clean_query);
    let prefix_query = format!("\"{}\"*", clean_query);

    let row = sqlx::query(
        r#"
            WITH fts_results AS (
                SELECT lexicon_id, rank, raw_text
                FROM lexicon_fts
                WHERE raw_text MATCH ?
                LIMIT 10
            )
            SELECT
                l.id, l.term_en, l.term_fr, l.category, f.rank,
                a.confidence as base_confidence
            FROM fts_results f
            JOIN lexicon l ON l.id = f.lexicon_id
            LEFT JOIN aliases a ON a.lexicon_id = l.id AND a.raw_text = f.raw_text
            ORDER BY
                (f.raw_text = ?) DESC,
                f.rank ASC
            LIMIT 1
    "#
    )
        .bind(&prefix_query)
        .bind(&clean_query)
        .fetch_optional(pool)
        .await
        .map_err(|_| Error::InternalServerError)?;

    if let Some(r) = row {
        let rank: f64 = r.get("rank");
        let base_conf: f32 = r.get::<Option<f64>, _>("base_confidence").unwrap_or(1.0) as f32;
        let final_confidence = if rank > -0.5 { base_conf * 0.75 } else { base_conf };

        Ok(Some(OcrMatchMetadata {
            raw_token: text.to_string(),
            lexicon_id: r.get::<i32, _>("id"),
            term_en: r.get::<String, _>("term_en"),
            term_fr: r.get::<String, _>("term_fr"),
            category: r.get::<Option<String>, _>("category").unwrap_or_else(|| "ingredient".to_string()),
            confidence: final_confidence,
            match_strategy: "fts5_fuzzy".to_string(),
        }))
    } else {
        Ok(None)
    }
}

pub fn parse_numeric(token: &str) -> Result<f32, ()> {
    let unicode_fractions = [
        ('¼', 0.25), ('½', 0.5), ('¾', 0.75), ('⅓', 0.333),
        ('⅔', 0.666), ('⅛', 0.125), ('⅜', 0.375), ('⅝', 0.625), ('⅞', 0.875)
    ];

    let re_mixed = Regex::new(r"(\d+)\s+(\d+)/(\d+)").unwrap();
    if let Some(caps) = re_mixed.captures(token) {
        let whole = caps[1].parse::<f32>().unwrap_or(0.0);
        let num = caps[2].parse::<f32>().unwrap_or(0.0);
        let den = caps[3].parse::<f32>().unwrap_or(1.0);
        return Ok(whole + (num / den));
    }

    for (c, val) in unicode_fractions {
        if token.contains(c) {
            let clean = token.replace(c, "").trim().to_string();
            let base = if clean.is_empty() { 0.0 } else { clean.parse::<f32>().unwrap_or(0.0) };
            return Ok(base + val);
        }
    }

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
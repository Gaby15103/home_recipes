use crate::dto::unit_dto::UnitDto;
use crate::recipe_parser::scanner::ScannedDocument;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LineType {
    Title,
    Header,
    Ingredient,
    Instruction,
    Fluff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedLine {
    pub raw_text: String,
    pub line_type: LineType,
    pub index: usize,
    pub confidence: f32,
}

pub struct DocumentClassifier<'a> {
    last_type: LineType,
    known_units: &'a [UnitDto],
    pool: &'a SqlitePool,
}

impl<'a> DocumentClassifier<'a> {
    pub fn new(units: &'a [UnitDto], pool: &'a SqlitePool) -> Self {
        Self {
            last_type: LineType::Fluff,
            known_units: units,
            pool,
        }
    }

    pub async fn segment_document(&mut self, doc: ScannedDocument) -> Vec<ClassifiedLine> {
        let mut results = Vec::new();
        for (i, line) in doc.raw_lines.into_iter().enumerate() {
            let mut classified = self.classify_single_line(line, i).await;
            classified.index = i;
            if classified.line_type != LineType::Fluff {
                results.push(classified);
            }
        }
        results
    }

    async fn classify_single_line(&mut self, line: String, index: usize) -> ClassifiedLine {
        let trimmed = line.trim();
        let lower = trimmed.to_lowercase();

        // 1. Kill the noise early
        if self.is_fluff(&lower) {
            return self.mark(line, LineType::Fluff, 1.0);
        }

        // 2. STAGE 1: Explicit Section Header check
        if self.is_section_marker(&lower).await {
            return self.mark(line, LineType::Header, 1.0);
        }

        // 3. STAGE 2: Instruction Detection
        if self.is_instruction_pattern(&trimmed).await {
            return self.mark(line, LineType::Instruction, 0.9);
        }

        // 4. STAGE 3: Ingredient Check
        let is_ing = self.is_ingredient_pattern(trimmed).await;
        if is_ing {
            return self.mark(line, LineType::Ingredient, 0.85);
        }

        // --- NEW LOGIC START ---
        // 5. STAGE 4: Continuation Check (The "Orphan" Recovery)
        // If the previous line was an ingredient, and this line is NOT a header/instruction,
        // and doesn't look like a title, it's likely a continuation (e.g., "haché finement").
        if self.last_type == LineType::Ingredient && index > 3 {
            return self.mark(line, LineType::Ingredient, 0.7);
        }
        // --- NEW LOGIC END ---

        // 6. STAGE 5: Title Check
        if index < 3 && trimmed.len() < 70 && !lower.ends_with(':') {
            return self.mark(line, LineType::Title, 0.8);
        }

        // 7. DEFAULT: Everything else is a step
        self.mark(line, LineType::Instruction, 0.5)
    }

    fn mark(&mut self, text: String, l_type: LineType, conf: f32) -> ClassifiedLine {
        self.last_type = l_type.clone();
        ClassifiedLine {
            raw_text: text,
            line_type: l_type,
            index: 0,
            confidence: conf,
        }
    }

    async fn is_ingredient_pattern(&self, line: &str) -> bool {
        let trimmed = line.trim().to_lowercase();

        // 1. Check for a unit anywhere in the line using FTS (very strong signal)
        // We bind "%trimmed%" or just the tokens to catch units buried after OCR noise
        let has_unit: i32 = sqlx::query_scalar(
            r#"
        SELECT COUNT(*) FROM lexicon_fts f
        JOIN lexicon l ON f.lexicon_id = l.id
        WHERE l.category = 'unit' AND lexicon_fts MATCH ?
        "#,
        )
            .bind(&trimmed)
            .fetch_one(self.pool)
            .await
            .unwrap_or(0);

        // 2. Look for any numeric/vulgar fraction
        let has_numeric = trimmed.chars().any(|c| c.is_ascii_digit() || "¼½¾⅓⅔⅛⅜⅝⅞".contains(c));

        // If it has a unit AND a number, it's an ingredient,
        // even if it has a weird prefix.
        if has_unit > 0 && has_numeric {
            return true;
        }

        // 3. Fallback for items like "1 citron" (no unit, just quantity)
        let first_word = trimmed.split_whitespace().next().unwrap_or("");
        if first_word.chars().next().map_or(false, |c| c.is_ascii_digit()) {
            // If it starts with a number and we aren't in a numbered list (1. 2.),
            // it's likely an ingredient
            if !self.is_instruction_pattern(line).await {
                return true;
            }
        }

        false
    }

    async fn is_instruction_pattern(&self, line: &str) -> bool {
        let trimmed = line.trim();
        let first_word = trimmed.split_whitespace().next().unwrap_or("");

        // Numbered list check (1., 2), etc.) - still logic-based
        let is_numbered = first_word
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_digit())
            && (first_word.ends_with('.') || first_word.ends_with(')'));

        if is_numbered {
            return true;
        }

        // DYNAMIC VERB CHECK: Look for 'action' category in the DB
        let first_token = first_word
            .to_lowercase()
            .replace(|c: char| !c.is_alphabetic(), "");
        let is_verb: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM lexicon l
         JOIN aliases a ON a.lexicon_id = l.id
         WHERE l.category = 'action' AND a.raw_text = ?",
        )
        .bind(&first_token)
        .fetch_one(self.pool)
        .await
        .unwrap_or(0);

        is_verb > 0
    }

    async fn is_section_marker(&self, lower: &str) -> bool {
        // DYNAMIC MARKER CHECK: Look for 'text' category (Ingredients, Préparation, etc.)
        for unit in self.known_units {
            if lower.contains(&unit.name_fr) {
                return false;
            }
        }

        let is_text_marker: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM lexicon l 
         JOIN aliases a ON a.lexicon_id = l.id 
         WHERE l.category = 'text' AND ? LIKE '%' || a.raw_text || '%'",
        )
        .bind(lower)
        .fetch_one(self.pool)
        .await
        .unwrap_or(0);

        is_text_marker > 0 || (lower.ends_with(':') && lower.len() < 30)
    }

    fn is_fluff(&self, lower: &str) -> bool {
        lower.len() < 2
            || lower.contains("copyright")
            || lower.starts_with("page")
            || lower.contains("www.")
    }
}

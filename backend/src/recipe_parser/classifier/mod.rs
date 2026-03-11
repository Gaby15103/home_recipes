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

        if self.is_fluff(&lower) {
            return self.mark(line, LineType::Fluff, 1.0);
        }

        // 1. Check for section markers (Ingrédients, Préparation)
        if self.is_section_marker(&lower).await {
            return self.mark(line, LineType::Header, 1.0);
        }

        // 2. Check for Instruction (Verbs or numbered lists)
        if self.is_instruction_pattern(&trimmed).await {
            return self.mark(line, LineType::Instruction, 0.9);
        }

        // 3. Check for Ingredients (Quantity + Uniform Symbol)
        if self.is_ingredient_pattern(trimmed).await {
            return self.mark(line, LineType::Ingredient, 0.85);
        }

        // 4. Orphan/Continuation Recovery
        // If the last line was an ingredient and this one is short/untyped, merge it.
        if self.last_type == LineType::Ingredient && index > 0 {
            if trimmed.len() < 40 && !trimmed.contains('.') {
                return self.mark(line, LineType::Ingredient, 0.7);
            }
        }

        // 5. Title Check (Document head)
        if index < 3 && trimmed.len() < 70 && !lower.ends_with(':') {
            return self.mark(line, LineType::Title, 0.8);
        }

        // 6. Default fallback
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
        let lower = line.trim().to_lowercase();

        // 1. FAST PATH: Check against your uniform English symbols (e.g., ml, g, tbsp)
        let has_unit = self.known_units.iter().any(|u| {
            let sym = u.symbol.to_lowercase();
            // Word boundary check: ensure 'g' doesn't match 'oignon'
            lower.split_whitespace().any(|word| {
                // Strip trailing periods from OCR (e.g., "5g." -> "5g")
                let clean_word = word.trim_end_matches('.');
                clean_word == sym || clean_word.ends_with(&sym) && clean_word.chars().next().map_or(false, |c| c.is_numeric())
            })
        });

        // 2. Identify numeric values or fractions
        let has_numeric = lower.chars().any(|c| c.is_ascii_digit() || "¼½¾⅓⅔⅛⅜⅝⅞".contains(c));

        if has_unit && has_numeric {
            return true;
        }

        // 3. Fallback for unitless items (e.g., "1 oignon", "2 poitrines")
        let first_word = lower.split_whitespace().next().unwrap_or("");
        if first_word.chars().next().map_or(false, |c| c.is_numeric()) {
            if !self.is_instruction_pattern(line).await {
                return true;
            }
        }

        false
    }

    async fn is_instruction_pattern(&self, line: &str) -> bool {
        let trimmed = line.trim();
        let first_word = trimmed.split_whitespace().next().unwrap_or("");

        // Check for "1." or "1)"
        let is_numbered = first_word.chars().next().map_or(false, |c| c.is_numeric())
            && (first_word.ends_with('.') || first_word.ends_with(')'));

        if is_numbered {
            return true;
        }

        // Check for French action verbs in your lexicon
        let first_token = first_word.to_lowercase().replace(|c: char| !c.is_alphabetic(), "");
        let is_verb: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM lexicon l JOIN aliases a ON a.lexicon_id = l.id WHERE l.category = 'action' AND a.raw_text = ?"
        )
            .bind(&first_token)
            .fetch_one(self.pool)
            .await
            .unwrap_or(0);

        is_verb > 0
    }

    async fn is_section_marker(&self, lower: &str) -> bool {
        // Ensure symbols like "t" (tbsp) aren't flagged as section headers
        for unit in self.known_units {
            if lower == unit.symbol.to_lowercase() {
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
            || lower.contains("http")
    }
}
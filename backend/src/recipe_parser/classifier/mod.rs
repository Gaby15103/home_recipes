use serde::{Deserialize, Serialize};
use crate::recipe_parser::scanner::ScannedDocument;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LineType {
    Title,       // Recipe name
    Header,      // Section headers like "For the sauce" or "Instructions"
    Ingredient,  // Individual ingredient lines
    Instruction, // Step-by-step paragraphs
    Fluff,       // Junk like "Page 1", "Copyright", or ads
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedLine {
    pub raw_text: String,
    pub line_type: LineType,
    pub index: usize,
    pub confidence: f32,
}

pub struct DocumentClassifier {
    last_type: LineType,
}

impl DocumentClassifier {
    pub fn new() -> Self {
        Self {
            last_type: LineType::Fluff,
        }
    }

    /// Primary entry point: Takes a ScannedDocument and returns tagged lines
    pub fn segment_document(&mut self, doc: ScannedDocument) -> Vec<ClassifiedLine> {
        doc.raw_lines
            .into_iter()
            .enumerate()
            .map(|(i, line)| self.classify_single_line(line, i))
            .filter(|l| l.line_type != LineType::Fluff) // Optional: remove fluff now or let frontend handle it
            .collect()
    }

    fn classify_single_line(&mut self, line: String, index: usize) -> ClassifiedLine {
        let trimmed = line.trim();
        let lower = trimmed.to_lowercase();

        // 1. RULE: IDENTIFY FLUFF (Highest Priority)
        if self.is_fluff(&lower) {
            return self.mark(line, LineType::Fluff, 1.0);
        }

        // 2. RULE: IDENTIFY TITLE
        // Usually the first short line that isn't fluff
        if index < 2 && trimmed.len() < 60 && !self.is_ingredient_pattern(trimmed) {
            return self.mark(line, LineType::Title, 0.9);
        }

        // 3. RULE: IDENTIFY EXPLICIT HEADERS
        if self.is_section_marker(&lower) {
            return self.mark(line, LineType::Header, 1.0);
        }

        // 4. RULE: IDENTIFY INGREDIENTS
        if self.is_ingredient_pattern(trimmed) {
            return self.mark(line, LineType::Ingredient, 0.85);
        }

        // 5. RULE: CONTEXTUAL GUESSING (The "Sticky" Logic)
        // If we were just in an ingredient block and find a short line, it's likely a sub-header
        let detected_type = if self.last_type == LineType::Ingredient && trimmed.len() < 35 {
            LineType::Header
        } else if trimmed.len() > 45 {
            LineType::Instruction
        } else {
            // Ambiguous short lines default to Instruction for safety,
            // but the user can calibrate this.
            LineType::Instruction
        };

        self.mark(line, detected_type, 0.6)
    }

    /// Helper to update the state and return the struct
    fn mark(&mut self, text: String, l_type: LineType, conf: f32) -> ClassifiedLine {
        self.last_type = l_type.clone();
        ClassifiedLine {
            raw_text: text,
            line_type: l_type,
            index: 0, // Will be set by enumerate in segment_document
            confidence: conf,
        }
    }

    fn is_ingredient_pattern(&self, line: &str) -> bool {
        // Starts with numbers, fractions, or Unicode vulgar fractions
        let has_qty = line.chars().next().map(|c| {
            c.is_ascii_digit() || "¼½¾⅓⅔⅛⅜⅝⅞".contains(c)
        }).unwrap_or(false);

        // Contains unit keywords
        let units = ["cup", "tbsp", "tsp", "gram", " ml ", " kg ", "clove", "pinch", "oz", " lb "];
        let has_unit = units.iter().any(|u| line.to_lowercase().contains(u));

        has_qty || has_unit
    }

    fn is_section_marker(&self, lower: &str) -> bool {
        lower.starts_with("prep")
            || lower.starts_with("ingred")
            || lower.starts_with("direct")
            || lower.starts_with("method")
            || lower.ends_with(':') && lower.len() < 30
    }

    fn is_fluff(&self, lower: &str) -> bool {
        lower.len() < 2
            || lower.contains("copyright")
            || lower.contains("all rights reserved")
            || lower.contains("www.")
            || lower.starts_with("page")
            || lower.contains("photo by")
    }
}
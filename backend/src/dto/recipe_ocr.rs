use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::dto::recipe_dto::{CreateRecipeInput, RecipeTranslationInput};
use crate::dto::ingredient_group_dto::{IngredientGroupInput, IngredientGroupTranslationInput};
use crate::dto::ingredient_dto::{IngredientInput, IngredientTranslationInput};
use crate::dto::step_group_dto::{StepGroupInput, StepGroupTranslationInput};
use crate::dto::step_dto::{StepInput, StepTranslationInput};
use crate::dto::tag_dto::InputTag;

// --- 1. OCR RAW DATA (Enhanced with dual-translation) ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrMatchMetadata {
    pub raw_token: String,
    pub lexicon_id: i32,
    pub term_en: String,
    pub term_fr: String, // Made mandatory to ensure consistency
    pub category: String, // "ingredient", "unit", "action"
    pub confidence: f32,
    pub match_strategy: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ParsedIngredientLine {
    pub quantity: Option<f32>,
    pub unit: Option<OcrMatchMetadata>,
    pub ingredient: Option<OcrMatchMetadata>,
    pub actions: Vec<OcrMatchMetadata>,
    pub original_line: String, // What OCR actually saw
    // Dual display names for the UI list
    pub display_name_en: String,
    pub display_name_fr: String,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrStep {
    pub position: i32,
    pub raw_text_en: String,
    pub raw_text_fr: String,
    pub detected_actions: Vec<OcrMatchMetadata>,
    pub detected_equipment: Vec<OcrMatchMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrIngredientGroup {
    pub name_en: String,
    pub name_fr: String,
    pub ingredients: Vec<ParsedIngredientLine>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrStepGroup {
    pub name_en: String,
    pub name_fr: String,
    pub steps: Vec<OcrStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrResultResponse {
    pub primary_language: String,
    pub title_en: String,
    pub title_fr: String,
    pub detected_servings: Option<i32>,
    pub ingredient_groups: Vec<OcrIngredientGroup>,
    pub step_groups: Vec<OcrStepGroup>,
    pub unparsed_segments: Vec<String>,
    pub raw_text: String,
}

// --- 2. THE WRAPPER (The payload from Frontend -> Backend) ---

#[derive(Debug, Deserialize)]
pub struct OcrCorrectionWrapper {
    /// The final recipe data to be saved in the main Application Database
    pub modified_recipe: CreateRecipeInput,
    /// Explicit "Learning" data to be saved in the SQLite dictionary.db
    pub lexicon_feedback: Vec<LexiconCorrection>,
}

#[derive(Debug, Deserialize)]
pub struct LexiconCorrection {
    /// The "broken" text (e.g., "h ile")
    pub raw_token: String,
    /// The ID in dictionary.db it should have matched (e.g., ID for "Huile")
    pub lexicon_id: i32,
    pub category: String,
}

// --- 3. THE CONFIRMATION DATA (User's manual fixes) ---

#[derive(Debug, Deserialize)]
pub struct OcrConfirmInput {
    pub translations: ComfirmRecipeTranslations,
    pub primary_language: String,
    pub image_url: String,
    pub author_id: Option<Uuid>,
    pub author: Option<String>,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub is_private: bool,
    pub tags: Vec<InputTag>,
    pub ingredient_groups: Vec<ConfirmIngredientGroup>,
    pub step_groups: Vec<ConfirmStepGroup>,
}
#[derive(Debug, Deserialize)]
pub struct ComfirmRecipeTranslations{
    pub language_code: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmIngredientGroup {
    pub translations: Vec<IngredientGroupTranslationInput>,
    pub ingredients: Vec<ConfirmIngredient>,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmStepGroup {
    pub translations: Vec<StepGroupTranslationInput>,
    pub steps: Vec<ConfirmStep>,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmIngredient {
    pub position: i32,
    pub quantity: f32,
    /// To handle the "Poulet" multi-line issue, we send back all lines involved
    pub source_ocr_lines: Vec<String>,
    /// NEW: The ID from your dictionary.db (SQLite) to teach the system
    pub confirmed_lexicon_id: Option<i32>,
    /// The ID used for the Main App DB (PostgreSQL)
    pub main_db_ingredient_id: Uuid,
    /// The ID used for the Main App DB (PostgreSQL)
    pub unit_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmStep {
    pub position: i32,
    pub text: String,
    /// Optional: track which OCR segments this step originated from
    pub source_ocr_segments: Vec<String>,
}


// --- 4. CONVERSION LOGIC (OCR -> App DB) ---

// --- 4. CONVERSION LOGIC (OCR -> App DB) ---

impl OcrConfirmInput {
    pub fn to_create_input(&self) -> CreateRecipeInput {
        CreateRecipeInput {
            primary_language: self.primary_language.clone(),
            image_url: self.image_url.clone(),
            servings: self.servings,
            prep_time_minutes: self.prep_time_minutes,
            cook_time_minutes: self.cook_time_minutes,
            author_id: self.author_id,
            author: self.author.clone(),
            is_private: self.is_private,
            tags: self.tags.clone(),
            translations: self.generate_translations(),
            ingredient_groups: self.map_ingredient_groups(),
            step_groups: self.map_step_groups(),
        }
    }

    fn generate_translations(&self) -> Vec<RecipeTranslationInput> {
        // Now accesses title through the translations struct
        vec![
            RecipeTranslationInput {
                language_code: "en".into(),
                title: self.translations.title.clone(),
                description: format!("Imported via OCR from {}", self.primary_language),
            },
            RecipeTranslationInput {
                language_code: "fr".into(),
                title: self.translations.title.clone(),
                description: format!("Importé via OCR depuis {}", self.primary_language),
            },
        ]
    }

    fn map_ingredient_groups(&self) -> Vec<IngredientGroupInput> {
        self.ingredient_groups
            .iter()
            .enumerate()
            .map(|(idx, group)| IngredientGroupInput {
                position: idx as i32,
                translations: group.translations.clone(),
                ingredients: group
                    .ingredients
                    .iter()
                    .map(|ing| IngredientInput {
                        position: ing.position,
                        quantity: Decimal::from_f32(ing.quantity).unwrap_or(Decimal::ZERO),
                        unit_id: ing.unit_id.unwrap_or_else(Uuid::nil),
                        translations: vec![IngredientTranslationInput {
                            language_code: self.primary_language.clone(),
                            // This ID refers to the Ingredient UUID in your main DB
                            data: ing.main_db_ingredient_id.to_string(),
                            note: Some(ing.source_ocr_lines.join(" ")),
                        }],
                    })
                    .collect(),
            })
            .collect()
    }

    fn map_step_groups(&self) -> Vec<StepGroupInput> {
        self.step_groups
            .iter()
            .enumerate()
            .map(|(idx, group)| StepGroupInput {
                position: idx as i32,
                translations: group.translations.clone(),
                steps: group
                    .steps
                    .iter()
                    .map(|s| StepInput {
                        position: s.position,
                        image_url: None,
                        duration_minutes: None,
                        translations: vec![StepTranslationInput {
                            language_code: self.primary_language.clone(),
                            instruction: s.text.clone(),
                        }],
                    })
                    .collect(),
            })
            .collect()
    }
}
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

// --- 1. OCR RAW DATA (What the parser found) ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrMatchMetadata {
    pub raw_token: String,
    pub lexicon_id: i32,
    pub term_en: String,
    pub term_fr: Option<String>,
    pub category: String,
    pub confidence: f32,
    pub match_strategy: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ParsedIngredientLine {
    pub quantity: Option<f32>,
    pub unit: Option<OcrMatchMetadata>,
    pub ingredient: Option<OcrMatchMetadata>,
    pub actions: Vec<OcrMatchMetadata>,
    pub original_line: String,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrStep {
    pub position: i32,
    pub raw_text: String,
    pub detected_actions: Vec<OcrMatchMetadata>,
    pub detected_equipment: Vec<OcrMatchMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrIngredientGroup {
    pub name: String,
    pub ingredients: Vec<ParsedIngredientLine>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrStepGroup {
    pub name: String,
    pub steps: Vec<OcrStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OcrResultResponse {
    pub primary_language: String,
    pub title: Option<String>,
    pub ingredient_groups: Vec<OcrIngredientGroup>,
    pub step_groups: Vec<OcrStepGroup>,
}

// --- 2. THE WRAPPER (The actual payload from Frontend) ---

#[derive(Debug, Deserialize)]
pub struct OcrCorrectionWrapper {
    /// The original OCR data (used to compare original_line vs final result for learning)
    pub original_ocr: OcrResultResponse,
    /// The final data modified by the user
    pub modified_recipe: OcrConfirmInput,
}

// --- 3. THE CONFIRMATION DATA (User's manual fixes) ---

#[derive(Debug, Deserialize)]
pub struct OcrConfirmInput {
    pub title: String,
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
    pub raw_ocr_line: String,      // Keep this to teach the lexicon
    pub confirmed_lexicon_id: i32,  // Keep this to teach the lexicon
    pub main_db_ingredient_id: Uuid,
    pub unit_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmStep {
    pub position: i32,
    pub text: String,
}

// --- 4. IMPLEMENTATIONS ---

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
        vec![
            RecipeTranslationInput { language_code: "en".into(), title: self.title.clone(), description: "".into() },
            RecipeTranslationInput { language_code: "fr".into(), title: self.title.clone(), description: "".into() },
        ]
    }

    fn map_ingredient_groups(&self) -> Vec<IngredientGroupInput> {
        self.ingredient_groups.iter().enumerate().map(|(idx, group)| {
            IngredientGroupInput {
                position: idx as i32,
                translations: group.translations.clone(),
                ingredients: group.ingredients.iter().map(|ing| {
                    IngredientInput {
                        position: ing.position,
                        quantity: Decimal::from_f32(ing.quantity).unwrap_or(Decimal::ZERO),
                        unit_id: ing.unit_id.unwrap_or_else(Uuid::nil),
                        translations: vec![
                            IngredientTranslationInput {
                                language_code: self.primary_language.clone(),
                                data: ing.main_db_ingredient_id.to_string(),
                                note: None,
                            }
                        ],
                    }
                }).collect(),
            }
        }).collect()
    }

    fn map_step_groups(&self) -> Vec<StepGroupInput> {
        self.step_groups.iter().enumerate().map(|(idx, group)| {
            StepGroupInput {
                position: idx as i32,
                translations: group.translations.clone(),
                steps: group.steps.iter().map(|s| {
                    StepInput {
                        position: s.position,
                        image_url: None,
                        duration_minutes: None,
                        translations: vec![
                            StepTranslationInput {
                                language_code: self.primary_language.clone(),
                                instruction: s.text.clone()
                            },
                        ],
                    }
                }).collect(),
            }
        }).collect()
    }
}
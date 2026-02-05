use serde::{Serialize, Deserialize};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::dto::TagResponse;
use crate::models::{Ingredient, IngredientTranslation, RecipeIngredient, RecipeIngredientTranslation, Tag};
use crate::utils::unit::IngredientUnit;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct IngredientTranslationInput {
    #[validate(length(min = 2, max = 5))]
    pub language: String,

    #[validate(length(min = 1, max = 50))]
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeIngredientTranslationInput {
    pub language_code: String,
    pub note: Option<String>,
}
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct IngredientInput {
    pub translations: Vec<IngredientTranslationInput>,
    pub quantity: BigDecimal,
    pub unit: IngredientUnit,
    pub note_translations: Option<RecipeIngredientTranslationInput>,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeIngredientTranslationResponse {
    pub language_code: String,
    pub note: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IngredientTranslationResponse {
    pub language_code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngredientResponse {
    pub id: Uuid,
    pub translations: Vec<IngredientTranslationResponse>,
    pub quantity: BigDecimal,
    pub unit: IngredientUnit,
    pub note: Option<String>,
    pub note_translations: Vec<RecipeIngredientTranslationResponse>,
    pub position: i32,
}


#[derive(Debug, Deserialize, Validate, Serialize,Clone)]
pub struct IngredientUpdate {
    pub id: Option<Uuid>,
    pub translations: Vec<IngredientTranslationInput>,
    pub quantity: BigDecimal,
    pub unit: IngredientUnit,
    pub note: Option<String>,
    pub note_translations: Option<RecipeIngredientTranslationInput>,
    pub position: i32,
}
impl From<(
    RecipeIngredient,
    Ingredient,
    Vec<IngredientTranslation>,
    Vec<RecipeIngredientTranslation>,
)> for IngredientResponse
{
    fn from(
        (ri, ing, name_translations, note_translations): (
            RecipeIngredient,
            Ingredient,
            Vec<IngredientTranslation>,
            Vec<RecipeIngredientTranslation>,
        ),
    ) -> Self {

        let name_translations_dto = name_translations
            .iter()
            .map(|t| IngredientTranslationResponse {
                language_code: t.language_code.clone(),
                name: t.name.clone(),
            })
            .collect::<Vec<_>>();

        let default_name = name_translations
            .iter()
            .find(|t| t.language_code == "fr")
            .or_else(|| name_translations.first())
            .map(|t| t.name.clone());

        let note_translations_dto = note_translations
            .iter()
            .map(|t| RecipeIngredientTranslationResponse {
                language_code: t.language_code.clone(),
                note: t.note.clone(),
            })
            .collect::<Vec<_>>();

        let default_note = note_translations
            .iter()
            .find(|t| t.language_code == "fr")
            .or_else(|| note_translations.first())
            .and_then(|t| t.note.clone());


        Self {
            id: ing.id,
            translations: name_translations_dto,
            quantity: ri.quantity,
            unit: ri.unit.parse().unwrap(),
            note: default_note,
            note_translations: note_translations_dto,
            position: ri.position,
        }
    }
}

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::dto::ingredient::{IngredientInput, IngredientResponse};
use crate::dto::IngredientUpdate;
use validator::Validate;
use crate::models::{IngredientGroup, IngredientGroupTranslation};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct IngredientGroupInput {
    pub translations: Vec<IngredientGroupTranslationInput>,
    pub position: i32,
    pub ingredients: Vec<IngredientInput>,
}

#[derive(Debug, Validate, Deserialize, Serialize,Clone)]
pub struct IngredientGroupTranslationInput {
    pub language: String,

    #[validate(length(min = 1, max = 100))]
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngredientGroupTranslationResponse {
    pub language_code: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "snake_case")]
pub struct IngredientGroupResponse {
    pub id: Uuid,

    pub title: String,
    pub translations: Vec<IngredientGroupTranslationResponse>,

    pub position: i32,
    pub ingredients: Vec<IngredientResponse>,
}


#[derive(Debug, Deserialize, Validate, Serialize,Clone)]
pub struct IngredientGroupUpdate {
    pub id: Option<Uuid>,
    pub translations: Vec<IngredientGroupTranslationInput>,
    pub position: i32,
    pub ingredients: Vec<IngredientUpdate>,
}
impl From<(
    IngredientGroup,
    Vec<IngredientGroupTranslation>,
    Vec<IngredientResponse>,
)> for IngredientGroupResponse
{
    fn from(
        (group, translations, ingredients): (
            IngredientGroup,
            Vec<IngredientGroupTranslation>,
            Vec<IngredientResponse>,
        ),
    ) -> Self {

        let translations_dto = translations
            .iter()
            .map(|t| IngredientGroupTranslationResponse {
                language_code: t.language_code.clone(),
                title: t.title.clone(),
            })
            .collect::<Vec<_>>();

        let default_title = translations
            .iter()
            .find(|t| t.language_code == "fr")
            .or_else(|| translations.first())
            .map(|t| t.title.clone())
            .unwrap_or_else(|| "".to_string());

        Self {
            id: group.id,
            title: default_title,
            translations: translations_dto,
            position: group.position,
            ingredients,
        }
    }
}

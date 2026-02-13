use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;


#[derive(Debug, Validate, Serialize, Deserialize, ToSchema, Clone)]
pub struct StepTranslationInput {
    pub language: String,
    pub instruction: String,
}
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct StepInput {
    pub position: i32,
    pub image_url: Option<String>,
    #[validate(nested)]
    pub translation: Vec<StepTranslationInput>,
    pub duration_minutes: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StepViewDto{
    pub id: Uuid,
    pub description: String,
    pub order: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StepEditorDto {
    pub id: Uuid,
    pub translations: Vec<StepTranslationsDto>,
    pub position: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StepTranslationsDto {
    pub id: Uuid,
    pub language_code: String,
    pub instruction: String,
}


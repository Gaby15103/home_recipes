use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;


#[derive(Debug, Validate, Serialize, Deserialize, ToSchema, Clone)]
pub struct StepTranslationInput {
    pub language_code: String,
    pub instruction: String,
}
#[derive(Debug, Validate, Serialize, Deserialize, ToSchema, Clone)]
pub struct EditStepTranslationInput {
    pub id: Option<Uuid>,
    pub language_code: String,
    pub instruction: String,
}
#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct StepInput {
    pub position: i32,
    pub image_url: Option<String>,
    #[validate(nested)]
    pub translations: Vec<StepTranslationInput>,
    pub duration_minutes: Option<i32>,
}
#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct EditStepInput {
    pub id: Option<Uuid>,
    pub position: i32,
    pub image_url: Option<String>,
    #[validate(nested)]
    pub translations: Vec<EditStepTranslationInput>,
    pub duration_minutes: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StepViewDto{
    pub id: Uuid,
    pub instruction: String,
    pub step_group_id: Uuid,
    pub position: i32,
    pub image_url: Option<String>,
    pub duration_minutes: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StepEditorDto {
    pub id: Uuid,
    pub step_group_id: Uuid,
    pub image_url: Option<String>,
    pub translations: Vec<StepTranslationsDto>,
    pub position: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StepTranslationsDto {
    pub id: Uuid,
    pub language_code: String,
    pub instruction: String,
}


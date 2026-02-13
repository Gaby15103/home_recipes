use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use crate::dto::step_dto::{StepEditorDto, StepInput, StepViewDto};
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct StepGroupInput {
    pub position: i32,
    pub translations: Vec<StepGroupTranslationInput>,
    #[validate(nested)]
    pub steps: Vec<StepInput>,
}
#[derive(Debug, Validate, Deserialize, Serialize, Clone, ToSchema)]
pub struct StepGroupTranslationInput {
    pub language_code: String,
    #[validate(length(min = 1, max = 100))]
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct StepGroupViewDto {
    pub id: Uuid,
    pub title: String,
    pub recipe_id: Uuid,
    pub position: i32,
    pub steps: Vec<StepViewDto>,
}
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StepGroupEditorDto {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub position: i32,
    pub steps: Vec<StepEditorDto>,
    pub translations: Vec<StepGroupTranslationDto>
}
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StepGroupTranslationDto{
    pub language_code: String,
    pub title: String,
}
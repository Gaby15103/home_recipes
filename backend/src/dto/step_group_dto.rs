use crate::dto::step_dto::{EditStepInput, StepEditorDto, StepInput, StepViewDto};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct StepGroupInput {
    pub position: i32,
    pub translations: Vec<StepGroupTranslationInput>,
    #[validate(nested)]
    pub steps: Vec<StepInput>,
}
#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct EditStepGroupInput {
    pub id: Option<Uuid>,
    pub position: i32,
    pub translations: Vec<EditStepGroupTranslationInput>,
    #[validate(nested)]
    pub steps: Vec<EditStepInput>,
}
#[derive(Debug, Validate, Deserialize, Serialize, Clone, ToSchema)]
pub struct StepGroupTranslationInput {
    pub language_code: String,
    #[validate(length(min = 1, max = 100))]
    pub title: String,
}
#[derive(Debug, Validate, Deserialize, Serialize, Clone, ToSchema)]
pub struct EditStepGroupTranslationInput {
    pub id: Option<Uuid>,
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
    pub id: Uuid,
    pub language_code: String,
    pub title: String,
}
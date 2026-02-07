use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::{Step, StepGroup, StepGroupTranslation, StepTranslation};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct StepTranslationInput {
    pub language: String,
    pub instruction: String,
}

#[derive(Debug, Validate, Deserialize,Serialize)]
pub struct StepInput {
    pub position: i32,
    pub translation: Vec<StepTranslationInput>,
    pub duration_minutes: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct StepGroupTranslationInput {
    pub language: String,
    pub title: String,
}
#[derive(Debug, Validate, Deserialize,Serialize)]
pub struct StepGroupInput {
    pub position: i32,
    pub translations: Vec<StepGroupTranslationInput>,
    pub steps: Vec<StepInput>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StepTranslationResponse {
    pub language: String,
    pub instruction: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StepGroupTranslationResponse {
    pub language: String,
    pub title: String,
}
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct StepResponse {
    pub id: Uuid,
    pub step_group_id: Uuid,
    pub position: i32,
    pub image_url: Option<String>,
    pub duration_minutes: Option<i32>,
    pub translations: Vec<StepTranslationResponse>,
}

#[derive(Debug, Deserialize, Validate, Serialize, Clone)]
pub struct StepUpdate {
    pub id: Option<Uuid>,
    pub position: i32,
    pub image_url: Option<String>,
    pub duration_minutes: Option<i32>,
    pub translations: Vec<StepTranslationInput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepGroupResponse {
    pub id: Uuid,
    pub position: i32,
    pub translations: Vec<StepGroupTranslationResponse>,
    pub steps: Vec<StepResponse>,
}

#[derive(Debug, Deserialize, Validate, Serialize,Clone)]
pub struct StepGroupUpdate {
    pub id: Option<Uuid>,
    pub position: i32,
    pub translations: Vec<StepGroupTranslationInput>,
    pub steps: Vec<StepUpdate>,
}

impl From<(Step, Vec<StepTranslation>)> for StepResponse {
    fn from((step, translations): (Step, Vec<StepTranslation>)) -> Self {

        let translation = translations
            .into_iter()
            .map(|t| StepTranslationResponse {
                language: t.language_code,
                instruction: t.instruction,
            })
            .collect();

        Self {
            id: step.id,
            step_group_id: step.step_group_id,
            position: step.position,
            duration_minutes: step.duration_minutes,
            image_url: step.image_url,
            translations: translation,
        }
    }
}
impl From<(StepGroup, Vec<StepGroupTranslation>, Vec<StepResponse>)>
for StepGroupResponse
{
    fn from(
        (group, translations, steps):
        (StepGroup, Vec<StepGroupTranslation>, Vec<StepResponse>)
    ) -> Self {

        let translations = translations
            .into_iter()
            .map(|t| StepGroupTranslationResponse {
                language: t.language_code,
                title: t.title,
            })
            .collect();

        Self {
            id: group.id,
            position: group.position,
            translations,
            steps,
        }
    }
}

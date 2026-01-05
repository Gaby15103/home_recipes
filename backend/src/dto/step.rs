use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::{Step, StepGroup};

#[derive(Debug, Validate, Deserialize,Serialize)]
pub struct StepInput {
    pub position: i32,
    pub instruction: String,
    pub duration_minutes: Option<i32>,
}

#[derive(Debug, Validate, Deserialize,Serialize)]
pub struct StepGroupInput {
    pub title: String,
    pub position: i32,
    pub steps: Vec<StepInput>,
}

#[derive(Debug, Serialize)]
pub struct StepResponse {
    pub id: Uuid,
    pub step_group_id: Uuid,
    pub position: i32,
    pub instruction: String,
    pub image_url: Option<String>,
    pub duration_minutes: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct StepGroupResponse {
    pub id: Uuid,
    pub title: String,
    pub position: i32,
    pub steps: Vec<StepResponse>,
}

impl StepGroupResponse {
    pub fn from_parts(
        step_group: StepGroup,
        steps: Vec<StepResponse>,
    ) -> Self {
        Self {
            id: step_group.id,
            title: step_group.title,
            position: step_group.position,
            steps,
        }
    }
}

impl From<Step> for StepResponse {
    fn from(step: Step) -> Self {
        Self {
            id: step.id,
            step_group_id: step.step_group_id,
            position: step.position,
            instruction: step.instruction,
            image_url: step.image_url,
            duration_minutes: step.duration_minutes,
        }
    }
}
impl From<StepGroup> for StepGroupResponse {
    fn from(step_group: StepGroup) -> Self {
        Self {
            id: step_group.id,
            title: step_group.title,
            position: step_group.position,
            steps: vec![],
        }
    }
}
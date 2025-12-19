use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::Tag;

#[derive(Debug, Validate, Serialize)]
pub struct TagResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputTag {
    Existing { id: Uuid },
    New { name: String },
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
        }
    }
}
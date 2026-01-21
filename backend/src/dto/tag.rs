use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::Tag;
use crate::utils::auth::Auth;
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct TagResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize,Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputTag {
    Existing { id: Uuid },
    New { name: String },
}

#[derive(Debug, Validate, serde_derive::Deserialize)]
pub struct CreateTag {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
}

#[derive(Debug)]
pub struct CreateTagOuter {
    pub auth: Auth,
    pub new_tag: CreateTag,
}

#[derive(Debug, Validate, serde_derive::Deserialize)]
pub struct UpdateTag {
    pub id: Uuid,
    #[validate(length(min = 1, max = 32))]
    pub name: String,
}
#[derive(Debug)]
pub struct UpdateTagOuter {
    pub auth: Auth,
    pub update_tag: UpdateTag,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
        }
    }
}
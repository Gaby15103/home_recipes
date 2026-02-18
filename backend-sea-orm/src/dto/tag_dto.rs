use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
use entity::tags;

#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct TagDto {
    pub id: Uuid,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema,Clone,)]
#[serde(tag = "type")]
pub enum InputTag {
    Existing { id: Uuid },
    New { name: String },
}
impl From<tags::Model> for TagDto {
    fn from(tags: tags::Model) -> Self {
        TagDto {
            id: tags.id,
            name: tags.name,
        }
    }
}
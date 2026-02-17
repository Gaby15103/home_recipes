use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct TagDto {
    pub id: Uuid,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub enum InputTag {
    Existing { id: Uuid },
    New { name: String },
}
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TagDto {
    pub id: Uuid,
    pub name: String,
}
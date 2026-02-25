use entity::roles;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema,Clone)]
pub struct RoleResponseDto {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
impl From<roles::Model> for RoleResponseDto {
    fn from(roles: roles::Model) -> Self {
        Self {
            id: roles.id,
            name: roles.name,
            description: roles.description,
        }
    }
}
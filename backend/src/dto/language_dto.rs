use entity::languages;
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct LanguageDto {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub is_active: bool,
    pub is_default: bool,
}
impl From<languages::Model> for LanguageDto {
    fn from(languages: languages::Model) -> Self {
        Self{
            code: languages.code,
            name: languages.name,
            native_name: languages.native_name,
            is_active: languages.is_active,
            is_default: languages.is_default,
        }
    }
}
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UserPreferences {
    pub language: String,
    pub theme: String,
}
impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            theme: "dark".to_string(),
        }
    }
}

use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UserPreferences {
    pub language: Option<String>,
    pub theme: Option<String>,
}
impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            language: Some("en".to_string()),
            theme: Some("dark".to_string()),
        }
    }
}

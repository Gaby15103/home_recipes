use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UserPreferences {
    pub language: String,
    pub theme: String,
    pub recipe_favorite_enabled: bool,
    pub recipe_comment_enabled: bool,
    pub comment_reply_enabled: bool,
}
impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            theme: "dark".to_string(),
            recipe_favorite_enabled: true,
            recipe_comment_enabled: true,
            comment_reply_enabled: true,
        }
    }
}

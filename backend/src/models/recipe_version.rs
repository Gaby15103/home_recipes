use chrono::DateTime;
use uuid::Uuid;
use crate::schema::{recipe_versions};

use crate::models::{User, Recipe};
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = recipe_versions)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(User, foreign_key = edited_by))]
pub struct RecipeVersion {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub data: serde_json::Value,
    pub edited_by: Uuid,
    pub created_at: DateTime<chrono::Utc>,
}
#[derive(Insertable)]
#[diesel(table_name = recipe_versions)]
pub struct NewRecipeVersion {
    pub recipe_id: Uuid,
    pub data: serde_json::Value,
    pub edited_by: Uuid,
}

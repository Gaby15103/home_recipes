use chrono::DateTime;
use serde_json::Value;
use uuid::Uuid;
use crate::schema::{recipe_analytics};

use crate::models::{User, Recipe};
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = recipe_analytics)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(User))]
pub struct RecipeAnalytics {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub user_id: Option<Uuid>,
    pub viewed_at: DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = recipe_analytics)]
pub struct NewRecipeAnalytics {
    pub recipe_id: Uuid,
    pub user_id: Option<Uuid>,
}

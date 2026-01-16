use chrono::DateTime;
use uuid::Uuid;
use crate::schema::{recipe_comments};

use crate::models::{User, Recipe};

#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(table_name = recipe_comments)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(RecipeComment, foreign_key = parent_id))]
pub struct RecipeComment {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
    pub edited_at: Option<DateTime<chrono::Utc>>,
    pub deleted_at: Option<DateTime<chrono::Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = recipe_comments)]
pub struct NewRecipeComment {
    pub recipe_id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
}

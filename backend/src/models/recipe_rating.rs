use chrono::DateTime;
use uuid::Uuid;
use crate::schema::{recipe_ratings};

use crate::models::{User, Recipe};
#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(primary_key(recipe_id, user_id))]
#[diesel(table_name = recipe_ratings)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(User))]
pub struct RecipeRating {
    pub recipe_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
    pub created_at: DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = recipe_ratings)]
pub struct NewRecipeRating {
    pub recipe_id: Uuid,
    pub user_id: Uuid,
    pub rating: i32,
}

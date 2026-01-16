use chrono::DateTime;
use uuid::Uuid;
use diesel::prelude::*;

use crate::schema::{favorites};

use crate::models::{User, Recipe};

#[derive(Debug, Clone, Queryable, Identifiable, Associations)]
#[diesel(primary_key(user_id, recipe_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Recipe, foreign_key = recipe_id))]
#[diesel(table_name = favorites)]
pub struct Favorite {
    pub user_id: Uuid,
    pub recipe_id: Uuid,
    pub created_at: DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = favorites)]
pub struct NewFavorite {
    pub user_id: Uuid,
    pub recipe_id: Uuid,
}
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{recipes};

// -----------------------------
// Recipe DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = recipes)]
pub struct Recipe {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = recipes)]
pub struct NewRecipe {
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = recipes)]
pub struct RecipeChange {
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
}
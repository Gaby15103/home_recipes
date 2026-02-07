use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{recipes};

// -----------------------------
// Recipe DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug, Deserialize, Serialize)]
#[diesel(table_name = recipes)]
pub struct Recipe {
    pub id: Uuid,
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub original_language_code: String,
}

// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = recipes)]
pub struct NewRecipe {
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
    pub image_url: String,
    pub servings: i32,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub author: String,
    pub author_id: Option<Uuid>,
    pub is_private: bool,
}
use crate::schema::{recipe_translations};
// -----------------------------
// RecipeTranslation DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Associations, Debug)]
#[diesel(table_name = recipe_translations)]
#[diesel(belongs_to(Recipe))]
#[derive(Clone)]
pub struct RecipeTranslation {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub language_code: String,
    pub title: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = recipe_translations)]
pub struct NewRecipeTranslation {
    pub recipe_id: Uuid,
    pub language_code: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = recipe_translations)]
pub struct RecipeTranslationChange {
    pub title: String,
    pub description: String,
}
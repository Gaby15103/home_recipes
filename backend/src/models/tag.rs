use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{tags};

// -----------------------------
// Tag DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}


// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = tags)]
pub struct TagChange {
    pub name: String,
}

use crate::schema::{recipe_tags};

// -----------------------------
// RecipeTag DB Model
// -----------------------------
#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = recipe_tags)]
pub struct RecipeTag {
    pub recipe_id: Uuid,
    pub tag_id: Uuid,
}

// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = recipe_tags)]
pub struct NewRecipeTag {
    pub recipe_id: Uuid,
    pub tag_id: Uuid,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = recipe_tags)]
pub struct RecipeRecipeTag {
    pub recipe_id: Uuid,
    pub tag_id: Uuid,
}
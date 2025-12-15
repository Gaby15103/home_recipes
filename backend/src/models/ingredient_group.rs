use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{ingredient_groups};

// -----------------------------
// Ingredient_groups DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = ingredient_groups)]
pub struct IngredientGroup {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub title: String,
    pub position: i32,
}

// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = ingredient_groups)]
pub struct NewIngredientGroup {
    pub recipe_id: Uuid,
    pub title: String,
    pub position: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = ingredient_groups)]
pub struct IngredientGroupChange {
    pub recipe_id: Uuid,
    pub title: String,
    pub position: i32,
}
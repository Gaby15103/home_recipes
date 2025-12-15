use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{ingredients};

// -----------------------------
// Ingredient DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = ingredients)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
}

// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = ingredients)]
pub struct NewIngredient {
    pub name: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = ingredients)]
pub struct IngredientChange {
    pub name: String,
}
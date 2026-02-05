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
}

use crate::schema::{ingredient_translations};

// -----------------------------
// IngredientTranslation
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Associations, Debug,Selectable)]
#[diesel(table_name = ingredient_translations)]
#[diesel(belongs_to(Ingredient))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IngredientTranslation {
    pub id: Uuid,
    pub ingredient_id: Uuid,
    pub language_code: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = ingredient_translations)]
pub struct NewIngredientTranslation {
    pub ingredient_id: Uuid,
    pub language_code: String,
    pub name: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = ingredient_translations)]
pub struct IngredientTranslationChange {
    pub name: String,
}

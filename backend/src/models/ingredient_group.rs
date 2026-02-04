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
    pub position: i32,
}

// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = ingredient_groups)]
pub struct NewIngredientGroup {
    pub recipe_id: Uuid,
    pub position: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = ingredient_groups)]
pub struct IngredientGroupChange {
    pub recipe_id: Uuid,
    pub position: i32,
}

use crate::schema::{ingredient_group_translations};

// -----------------------------
// IngredientGroupTranslation
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Associations, Debug)]
#[diesel(table_name = ingredient_group_translations)]
#[diesel(belongs_to(IngredientGroup))]
pub struct IngredientGroupTranslation {
    pub id: Uuid,
    pub ingredient_group_id: Uuid,
    pub language_code: String,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = ingredient_group_translations)]
pub struct NewIngredientGroupTranslation {
    pub ingredient_group_id: Uuid,
    pub language_code: String,
    pub title: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = ingredient_group_translations)]
pub struct IngredientGroupTranslationChange {
    pub title: String,
}
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;
use bigdecimal::BigDecimal;

use crate::schema::{recipe_ingredients};

// -----------------------------
// RecipeIngredient DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = recipe_ingredients)]
pub struct RecipeIngredient {
    pub id: Uuid,
    pub ingredient_group_id: Uuid,
    pub ingredient_id: Uuid,
    pub quantity: BigDecimal,
    pub unit: String,
    pub note: Option<String>,
    pub position: i32,
}

// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = recipe_ingredients)]
pub struct NewRecipeIngredient {
    pub ingredient_group_id: Uuid,
    pub ingredient_id: Uuid,
    pub quantity: BigDecimal,
    pub unit: String,
    pub note: Option<String>,
    pub position: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = recipe_ingredients)]
pub struct RecipeIngredientChange {
    pub ingredient_group_id: Uuid,
    pub ingredient_id: Uuid,
    pub quantity: BigDecimal,
    pub unit: String,
    pub note: Option<String>,
    pub position: i32,
}
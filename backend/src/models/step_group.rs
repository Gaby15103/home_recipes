use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{step_groups};

// -----------------------------
// StepGroup DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug, Selectable)]
#[diesel(table_name = step_groups)]
pub struct StepGroup {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub title: String,
    pub position: i32,
}
// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = step_groups)]
pub struct NewStepGroup {
    pub recipe_id: Uuid,
    pub title: String,
    pub position: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = step_groups)]
pub struct StepGroupChange {
    pub recipe_id: Uuid,
    pub title: String,
    pub position: i32,
}
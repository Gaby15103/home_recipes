use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{steps};

// -----------------------------
// Step DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = steps)]
pub struct Step {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub step_group_id: Uuid,
    pub position: i32,
    pub instruction: String,
    pub duration_minutes: i32
}
// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = steps)]
pub struct NewStep {
    pub recipe_id: Uuid,
    pub step_group_id: Uuid,
    pub position: i32,
    pub instruction: String,
    pub duration_minutes: i32
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = steps)]
pub struct StepChange {
    pub recipe_id: Uuid,
    pub step_group_id: Uuid,
    pub position: i32,
    pub instruction: String,
    pub duration_minutes: i32
}
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
    pub position: i32,
}
// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = step_groups)]
pub struct NewStepGroup {
    pub recipe_id: Uuid,
    pub position: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = step_groups)]
pub struct StepGroupChange {
    pub recipe_id: Uuid,
    pub position: i32,
}

use crate::schema::{step_group_translations};

// -----------------------------
// StepGroupTranslation DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Associations, Debug)]
#[diesel(table_name = step_group_translations)]
#[diesel(belongs_to(StepGroup))]
pub struct StepGroupTranslation {
    pub id: Uuid,
    pub step_group_id: Uuid,
    pub language_code: String,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = step_group_translations)]
pub struct NewStepGroupTranslation {
    pub step_group_id: Uuid,
    pub language_code: String,
    pub title: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = step_group_translations)]
pub struct StepGroupTranslationChange {
    pub title: String,
}
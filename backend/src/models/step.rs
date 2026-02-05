use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{steps};

// -----------------------------
// Step DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug, Selectable)]
#[diesel(table_name = steps)]
pub struct Step {
    pub id: Uuid,
    pub step_group_id: Uuid,
    pub position: i32,
    pub image_url: Option<String>,
    pub duration_minutes: Option<i32>
}
// Insertable for creating new recipes
#[derive(Insertable)]
#[diesel(table_name = steps)]
pub struct NewStep {
    pub step_group_id: Uuid,
    pub position: i32,
    pub image_url: Option<String>,
    pub duration_minutes: i32
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = steps)]
pub struct StepChange {
    pub step_group_id: Uuid,
    pub position: i32,
    pub image_url: Option<String>,
    pub duration_minutes: Option<i32>
}

use crate::schema::{step_translations};

// -----------------------------
// StepTranslation DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Associations, Debug,Selectable,)]
#[diesel(table_name = step_translations)]
#[diesel(belongs_to(Step))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StepTranslation {
    pub id: Uuid,
    pub step_id: Uuid,
    pub language_code: String,
    pub instruction: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = step_translations)]
pub struct NewStepTranslation {
    pub step_id: Uuid,
    pub language_code: String,
    pub instruction: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = step_translations)]
pub struct StepTranslationChange {
    pub instruction: String,
}
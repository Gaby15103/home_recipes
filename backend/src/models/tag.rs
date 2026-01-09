use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::{tags};

// -----------------------------
// Tag DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug, Selectable)]
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
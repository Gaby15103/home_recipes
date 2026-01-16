use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{users};

// -----------------------------
// Preferences DTO
// -----------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub language: Option<String>,
    pub theme: Option<String>,
}

// -----------------------------
// User DB Model
// -----------------------------
#[derive(Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,

    pub first_name: String,
    pub last_name: String,

    pub password_hash: String,

    pub avatar_url: Option<String>,
    pub preferences: serde_json::Value,

    pub is_active: Option<bool>,
    pub email_verified: Option<bool>,

    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    pub two_factor_secret: Option<String>,
    pub two_factor_recovery_codes: Option<serde_json::Value>,
    pub two_factor_confirmed_at: Option<chrono::NaiveDateTime>,
}

// Insertable for creating new users
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub preferences: serde_json::Value,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserChange {
    pub email: Option<String>,
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub preferences: serde_json::Value,
}
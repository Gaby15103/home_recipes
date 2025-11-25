use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::pg::Pg;
use crate::schema::{users, roles, user_roles};

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
#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,

    pub preferences: serde_json::Value,

    pub is_active: Option<bool>,
    pub email_verified: Option<bool>,

    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Insertable for creating new users
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password_hash: &'a str,
    pub display_name: Option<&'a str>,
    pub avatar_url: Option<&'a str>,
    pub preferences: &'a serde_json::Value,
}

// -----------------------------
// Role DB Model
// -----------------------------
#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = roles)]
#[diesel(check_for_backend(Pg))]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

// Insertable for creating new roles
#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}

// -----------------------------
// UserRole DB Model
// -----------------------------
#[derive(Queryable, Selectable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = user_roles)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: i32,
}

// Insertable for linking users and roles
#[derive(Insertable)]
#[diesel(table_name = user_roles)]
pub struct NewUserRole {
    pub user_id: Uuid,
    pub role_id: i32,
}

// -----------------------------
// DTO: User with roles
// -----------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithRoles {
    pub user: User,
    pub roles: Vec<Role>,
}

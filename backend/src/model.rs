use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub language: Option<String>,
    pub theme: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String       ,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,

    pub preferences: serde_json::Value,

    pub is_active: Option<bool>,
    pub email_verified: Option<bool>,

    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,

    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: i32,
}
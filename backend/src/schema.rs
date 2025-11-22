use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// JSON structure for creating a new user
#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreateSchema {
    pub email: String,
    pub username: String,

    // Local auth
    pub password: String,

    // Optional profile info
    pub display_name: String,
    pub avatar_url: String,

    pub preferences: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateSchema {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub preferences: Option<Value>,
}


/// JSON structure sent back to the client
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponseSchema {
    pub id: Uuid,
    pub email: String,
    pub username: String,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub preferences: Value,

    pub is_active: bool,
    pub email_verified: bool,

    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}



/// Create a new role
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleCreateSchema {
    pub name: String,
    pub description: Option<String>,
}

/// Update an existing role
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleUpdateSchema {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Response sent to client
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleResponseSchema {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

/// Assign a role to a user
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleCreateSchema {
    pub user_id: Uuid,
    pub role_id: i32,
}

/// Response sent to client
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleResponseSchema {
    pub user_id: Uuid,
    pub role_id: i32,
}
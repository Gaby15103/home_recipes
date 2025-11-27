use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::pg::Pg;
use crate::schema::{users, roles, user_roles};
use crate::schema::users::first_name;





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

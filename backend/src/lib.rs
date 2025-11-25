use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;
pub mod auth;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewUser, User};

pub fn create_user(
    conn: &mut PgConnection,
    email: &str,
    username: &str,
    password_hash: &str,
    first_name: &str,
    last_name: &str,
    avatar_url: Option<&str>,
    preferences: &serde_json::Value,
) -> User {
    use crate::schema::users;
    use crate::schema::user_roles;
    use crate::models::NewUserRole;
    use crate::schema::roles::dsl::*;

    let new_user = NewUser {
        email,
        username,
        password_hash,
        first_name,
        last_name,
        avatar_url,
        preferences,
    };

    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user");

    let user_role_id: i32 = roles
        .filter(name.eq("USER"))
        .select(id)
        .first(conn)
        .expect("USER role not found");

    // Step 3: insert into user_roles join table
    let new_user_role = NewUserRole {
        user_id: user.id,
        role_id: user_role_id,
    };

    diesel::insert_into(user_roles::table)
        .values(&new_user_role)
        .execute(conn)
        .expect("Failed to assign USER role");

    user
}

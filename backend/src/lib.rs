use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

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
    display_name: Option<&str>,
    avatar_url: Option<&str>,
    preferences: &serde_json::Value,
) -> User {
    use crate::schema::users;

    let new_user = NewUser {
        email,
        username,
        password_hash,
        display_name,
        avatar_url,
        preferences,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}

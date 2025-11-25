use backend::*;
use serde_json::json;
use std::io::{Read, Write, stdin, stdout};
use crate::auth::hash::hash_password;

fn main() {
    let connection = &mut establish_connection();

    let mut email = String::new();
    print!("Enter your email: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut email).unwrap();
    let email = email.trim_end();

    let mut first_name = String::new();
    print!("Enter your first name: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut first_name).unwrap();
    let first_name = first_name.trim_end();

    let mut last_name = String::new();
    print!("Enter your last name: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut last_name).unwrap();
    let last_name = last_name.trim_end();

    let mut username = String::new();
    print!("Enter your username: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end();

    let mut password = String::new();
    print!("Enter your password: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end();
    let password_hash = &hash_password(password);


    let mut avatar_url = String::new();
    print!("Enter your avatar URL (optional): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut avatar_url).unwrap();
    let avatar_url = avatar_url.trim_end();
    let avatar_url = if avatar_url.is_empty() {
        None
    } else {
        Some(avatar_url)
    };

    // For preferences, let's just use default JSON for now
    let preferences = &json!({
        "language": "en",
        "theme": "light"
    });

    let new_user = create_user(
        connection,
        email,
        username,
        password_hash,
        first_name,
        last_name,
        avatar_url,
        preferences,
    );
    println!("\nSaved new User with uuid {}", new_user.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
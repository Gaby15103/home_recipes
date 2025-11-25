use backend::*;
use serde_json::json;
use std::io::{Read, Write, stdin, stdout};

fn main() {
    let connection = &mut establish_connection();

    let mut email = String::new();
    print!("Enter your email: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut email).unwrap();
    let email = email.trim_end();

    let mut username = String::new();
    print!("Enter your username: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end();

    let mut password_hash = String::new();
    print!("Enter your password hash: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut password_hash).unwrap();
    let password_hash = password_hash.trim_end();

    let mut display_name = String::new();
    print!("Enter your display name (optional): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut display_name).unwrap();
    let display_name = display_name.trim_end();
    let display_name = if display_name.is_empty() {
        None
    } else {
        Some(display_name)
    };

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
        display_name,
        avatar_url,
        preferences,
    );
    println!("\nSaved new User with uuid {}", new_user.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
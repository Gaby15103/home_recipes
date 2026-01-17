use std::env;
use crate::models::User;
use lettre::message::Mailbox;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use crate::dto::UserResponseInner;

/// Sends an email confirmation to the user
pub fn send_email_confirmation(user: UserResponseInner, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok().unwrap();
    let email_body = format!(
        r#"
Hello {username},

Welcome to HomeRecipes! Click the link below to verify your email:

{frontend_origin}/verify?token={token}

If you didn't create an account, ignore this email.

Thanks,
The HomeRecipes Team
"#,
        username = user.username,
        token = token
    );

    let email = Message::builder()
        .from("no-reply@homerecipes.com".parse::<Mailbox>()?)
        .to(user.email.parse::<Mailbox>()?)
        .subject("HomeRecipes - Verify your email")
        .body(email_body)?;

    let creds = Credentials::new(
        std::env::var("SMTP_USER")?,
        std::env::var("SMTP_PASS")?,
    );

    let mailer = SmtpTransport::relay(&std::env::var("SMTP_HOST")?)?
        .credentials(creds)
        .build();

    mailer.send(&email)?;
    println!("✅ Confirmation email sent to {}", user.email);
    Ok(())
}

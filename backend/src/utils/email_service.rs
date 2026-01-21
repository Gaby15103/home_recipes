use std::env;
use crate::dto::UserResponseInner;
use crate::error::Error;
use lettre::message::Mailbox;
use lettre::{Message, SmtpTransport, Transport};

/// Sends an email confirmation to the user
pub fn send_email_confirmation(user: UserResponseInner, token: &str) -> Result<(), Error> {
    let frontend_origin = env::var("FRONTEND_ORIGIN")
        .map_err(|_| Error::EmailSend(json!({"error": "Missing FRONTEND_ORIGIN"})))?;

    let email_body = format!(
        "Hello {},\nWelcome to HomeRecipes! Click the link below to verify your email:\n\n\
     Click to verify your email: {}/verify?token={}\
     \nIf you didn't create an account, ignore this email.\n\nThanks,\nThe HomeRecipes Team",
        user.username,
        frontend_origin,
        token.replace("\n", "") // make sure no newlines break the token
    );


    let email = Message::builder()
        .from(
            env::var("MAIL_FROM_ADDRESS")
                .unwrap_or_else(|_| "no-reply@homerecipes.com".to_string())
                .parse::<Mailbox>()
                .map_err(|e| Error::EmailSend(json!({"error": e.to_string()})))?
        )
        .to(
            user.email
                .parse::<Mailbox>()
                .map_err(|e| Error::EmailSend(json!({"error": e.to_string()})))?
        )
        .subject("HomeRecipes - Verify your email")
        .body(email_body)
        .map_err(|e| Error::EmailSend(json!({"error": e.to_string()})))?;

    // Read host/port
    let smtp_host = env::var("SMTP_HOST")
        .unwrap_or_else(|_| "localhost".to_string());
    let smtp_port: u16 = env::var("SMTP_PORT")
        .unwrap_or_else(|_| "1025".to_string())
        .parse()
        .map_err(|_| Error::EmailSend(json!({"error": "Invalid SMTP_PORT"})))?;

    // MailHog doesn't require authentication
    let mailer = SmtpTransport::builder_dangerous(&smtp_host)
        .port(smtp_port)
        .build();

    mailer.send(&email)
        .map_err(|e| Error::EmailSend(json!({"error": e.to_string()})))?;

    Ok(())
}

use crate::errors::Error;
use crate::logging::error_logger::ErrorLogger;
use entity::users;
use lettre::message::{header::ContentType, Mailbox, MultiPart, SinglePart};
use lettre::{Message, SmtpTransport, Transport};
use serde_json::json;
use std::env;
use uuid::Uuid;
use crate::logging::ErrorLog;

pub fn send_email_confirmation(user: users::Model, token: &Uuid) -> Result<(), Error> {
    let frontend_origin = get_frontend_origin()?;
    let verify_url = format!("{}/verify?token={}", frontend_origin, token);

    let html_body = format!(
        "<html><body style='font-family: sans-serif; color: #18181b;'>\
        <h3>Hello {},</h3>\
        <p>Welcome to <b>HomeRecipes</b>! Please click the button below to verify your email address:</p>\
        <div style='margin: 24px 0;'>\
            <a href='{}' style='background-color: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 8px; font-weight: bold; display: inline-block;'>Verify Email</a>\
        </div>\
        <p style='font-size: 12px; color: #71717a;'>If the button doesn't work, copy and paste this link:<br/>{}</p>\
        <p>Thanks,<br/>The HomeRecipes Team</p>\
        </body></html>",
        user.username, verify_url, verify_url
    );

    let text_body = format!(
        "Hello {},\nWelcome to HomeRecipes! Click the link below to verify your email:\n\n{}\n\nIf you didn't create an account, ignore this email.\n\nThanks,\nThe HomeRecipes Team",
        user.username, verify_url
    );

    let email = Message::builder()
        .from(get_mailbox("MAIL_FROM_ADDRESS", "no-reply@homerecipes.com")?)
        .to(parse_recipient(&user.email)?)
        .subject("HomeRecipes - Verify your email")
        .multipart(
            MultiPart::alternative()
                .singlepart(SinglePart::plain(text_body))
                .singlepart(SinglePart::html(html_body)),
        )
        .map_err(|e| log_email_err("Message Building", e.to_string(), &user.email))?;

    send_via_smtp(email)
}

pub fn send_password_reset(user: users::Model, token: &Uuid) -> Result<(), Error> {
    let frontend_origin = get_frontend_origin()?;
    let reset_url = format!("{}/reset-password?token={}", frontend_origin, token);

    let html_body = format!(
        "<html><body style='font-family: sans-serif; color: #18181b;'>\
        <h3>Hello {},</h3>\
        <p>We received a request to reset your password. Click the button below to set a new one:</p>\
        <div style='margin: 24px 0;'>\
            <a href='{}' style='background-color: #dc2626; color: white; padding: 12px 24px; text-decoration: none; border-radius: 8px; font-weight: bold; display: inline-block;'>Reset Password</a>\
        </div>\
        <p>This link will expire in 1 hour.</p>\
        <p style='font-size: 12px; color: #71717a;'>If you didn't request this, you can safely ignore this email.</p>\
        <p>Thanks,<br/>The HomeRecipes Team</p>\
        </body></html>",
        user.username, reset_url
    );

    let text_body = format!(
        "Hello {},\n\nReset your password here: {}\n\nThis link expires in 1 hour.",
        user.username, reset_url
    );

    let email = Message::builder()
        .from(get_mailbox("MAIL_FROM_ADDRESS", "no-reply@homerecipes.com")?)
        .to(parse_recipient(&user.email)?)
        .subject("HomeRecipes - Password Reset")
        .multipart(
            MultiPart::alternative()
                .singlepart(SinglePart::plain(text_body))
                .singlepart(SinglePart::html(html_body)),
        )
        .map_err(|e| log_email_err("Message Building", e.to_string(), &user.email))?;

    send_via_smtp(email)
}

// --- Internal Helpers ---

fn get_frontend_origin() -> Result<String, Error> {
    env::var("FRONTEND_ORIGIN").map_err(|_| {
        log_email_err(
            "Config Check",
            "Missing FRONTEND_ORIGIN".to_string(),
            "ENV",
        )
    })
}

fn get_mailbox(env_var: &str, default: &str) -> Result<Mailbox, Error> {
    env::var(env_var)
        .unwrap_or_else(|_| default.to_string())
        .parse::<Mailbox>()
        .map_err(|e| log_email_err("Mailbox Parse", e.to_string(), env_var))
}

fn parse_recipient(email: &str) -> Result<Mailbox, Error> {
    email
        .parse::<Mailbox>()
        .map_err(|e| log_email_err("Recipient Parse", e.to_string(), email))
}

fn send_via_smtp(email: Message) -> Result<(), Error> {
    let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string());
    let smtp_port: u16 = env::var("SMTP_PORT")
        .unwrap_or_else(|_| "1025".to_string())
        .parse()
        .map_err(|_| Error::InternalServerError(json!({"error": "Invalid SMTP_PORT"})))?;

    let mailer = SmtpTransport::builder_dangerous(&smtp_host)
        .port(smtp_port)
        .build();

    mailer.send(&email).map_err(|e| {
        log_email_err("SMTP Transmission", e.to_string(), &smtp_host)
    })?;

    Ok(())
}

fn log_email_err(op: &str, err: String, target: &str) -> Error {
    let logger = ErrorLogger::from_env();

    // Build ErrorLog using your specific builder pattern
    let log_entry = ErrorLog::new("EmailServiceError", format!("Email error during: {}", op))
        .with_metadata("operation".to_string(), op.to_string())
        .with_metadata("error_details".to_string(), err.clone())
        .with_metadata("target".to_string(), target.to_string());

    let error_id = logger.log(&log_entry);

    Error::InternalServerError(json!({
        "error_id": error_id,
        "message": format!("Email service failure: {}", op),
        "details": err
    }))
}
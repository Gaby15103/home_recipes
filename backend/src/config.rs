use std::env;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_address: String,
    pub jwt_secret_key: String,
    pub frontend_origin: String,

    pub redis_url: String,

    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: Option<String>,
    pub smtp_pass: Option<String>,
    pub mail_from_address: String,
    pub mail_from_name: String,

    pub gemini_api_key: String,
    pub groq_cloud_api_key: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable: {0}")]
    Missing(&'static str),

    #[error("invalid value for {0}")]
    Invalid(&'static str),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| ConfigError::Missing("DATABASE_URL"))?;

        let bind_address =
            env::var("BIND_ADDRESS").map_err(|_| ConfigError::Missing("BIND_ADDRESS"))?;

        let jwt_secret_key =
            env::var("JWT_SECRET_KEY").map_err(|_| ConfigError::Missing("JWT_SECRET_KEY"))?;

        let frontend_origin =
            env::var("FRONTEND_ORIGIN").map_err(|_| ConfigError::Missing("FRONTEND_ORIGIN"))?;

        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());

        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string());

        let smtp_port = env::var("SMTP_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(1025);

        let smtp_user = env::var("SMTP_USER").ok().filter(|s| !s.is_empty());
        let smtp_pass = env::var("SMTP_PASS").ok().filter(|s| !s.is_empty());

        let mail_from_address = env::var("MAIL_FROM_ADDRESS")
            .unwrap_or_else(|_| "no-reply@homerecipes.com".to_string());

        let mail_from_name =
            env::var("MAIL_FROM_NAME").unwrap_or_else(|_| "HomeRecipes".to_string());

        let gemini_api_key = env::var("GEMINI_API_KEY").unwrap_or_else(|_| "GEMINI_API_KAY".to_string());
        
        let groq_cloud_api_key = env::var("GROQ_CLOUD_API_KEY").unwrap_or_else(|_| "GROQ_CLOUD_API_KEY".to_string());


        Ok(Self {
            database_url,
            bind_address,
            jwt_secret_key,
            frontend_origin,
            redis_url,
            smtp_host,
            smtp_port,
            smtp_user,
            smtp_pass,
            mail_from_address,
            mail_from_name,
            gemini_api_key,
            groq_cloud_api_key
        })
    }
}

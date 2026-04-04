use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Complete error context for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLog {
    /// Unique error ID for tracking across logs
    pub error_id: String,

    /// High-level error type
    pub error_type: String,

    /// Full error message with details
    pub message: String,

    /// Stack trace or error chain
    pub stack_trace: Option<String>,

    /// HTTP context
    pub http_context: Option<HttpContext>,

    /// Database context (if applicable)
    pub db_context: Option<DbContext>,

    /// File/Parser context (your recipe parser issue!)
    pub parser_context: Option<ParserContext>,

    /// Environment info (helps debug prod vs dev)
    pub environment: EnvironmentInfo,

    /// When the error occurred
    pub timestamp: DateTime<Utc>,

    /// Arbitrary metadata for extra context
    pub metadata: HashMap<String, String>,
}

/// HTTP request/response context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpContext {
    pub method: String,
    pub path: String,
    pub status_code: u16,
    pub user_id: Option<String>,
    pub query_params: Option<String>,
}

/// Database operation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbContext {
    pub operation: String, // "INSERT", "SELECT", "UPDATE", etc.
    pub table: String,
    pub error_msg: String,
}

/// Parser-specific context (for recipe file parsing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserContext {
    pub parser_type: String, // e.g., "RecipeFileParser"
    pub file_name: Option<String>,
    pub file_size: Option<u64>,
    pub file_format: Option<String>,
    pub parse_stage: String, // e.g., "validation", "serialization"
    pub input_sample: Option<String>, // First 500 chars of problematic input
}

/// Environment information for debugging prod vs local
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    pub environment: String, // "production", "development", etc.
    pub rust_version: String,
    pub app_version: String,
    pub host: String,
    pub database_type: String,
}

impl ErrorLog {
    pub fn new(error_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error_id: Uuid::new_v4().to_string(),
            error_type: error_type.into(),
            message: message.into(),
            stack_trace: None,
            http_context: None,
            db_context: None,
            parser_context: None,
            environment: EnvironmentInfo::current(),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_http_context(mut self, ctx: HttpContext) -> Self {
        self.http_context = Some(ctx);
        self
    }

    pub fn with_db_context(mut self, ctx: DbContext) -> Self {
        self.db_context = Some(ctx);
        self
    }

    pub fn with_parser_context(mut self, ctx: ParserContext) -> Self {
        self.parser_context = Some(ctx);
        self
    }

    pub fn with_stack_trace(mut self, trace: String) -> Self {
        self.stack_trace = Some(trace);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl EnvironmentInfo {
    pub fn current() -> Self {
        Self {
            environment: std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            rust_version: env!("CARGO_PKG_RUST_VERSION").to_string(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            host: std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string()),
            database_type: "postgres".to_string(),
        }
    }
}
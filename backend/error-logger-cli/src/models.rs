use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLog {
    pub error_id: String,
    pub error_type: String,
    pub message: String,
    pub stack_trace: Option<String>,
    pub http_context: Option<HttpContext>,
    pub db_context: Option<DbContext>,
    pub parser_context: Option<ParserContext>,
    pub environment: EnvironmentInfo,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpContext {
    pub method: String,
    pub path: String,
    pub status_code: u16,
    pub user_id: Option<String>,
    pub query_params: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbContext {
    pub operation: String,
    pub table: String,
    pub error_msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserContext {
    pub parser_type: String,
    pub file_name: Option<String>,
    pub file_size: Option<u64>,
    pub file_format: Option<String>,
    pub parse_stage: String,
    pub input_sample: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    pub environment: String,
    pub rust_version: String,
    pub app_version: String,
    pub host: String,
    pub database_type: String,
}
use actix::MailboxError;
use actix_web::{
    error::{JsonPayloadError, PayloadError, QueryPayloadError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use sea_orm::{DbErr, TransactionError};
use serde_json::{json, Map as JsonMap, Value as JsonValue};
use std::io;
use thiserror::Error;
use validator::ValidationErrors;
use crate::logging::{ErrorLog, ParserContext, DbContext, HttpContext, ErrorLogger};
/* -------------------------------------------------------------------------- */
/*                               APP ERROR TYPE                               */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Error)]
pub enum Error {
    #[error("Bad request")]
    BadRequest(JsonValue),

    #[error("Unauthorized")]
    Unauthorized(JsonValue),

    #[error("Forbidden")]
    Forbidden(JsonValue),

    #[error("Not found")]
    NotFound(JsonValue),

    #[error("Unprocessable entity")]
    UnprocessableEntity(JsonValue),

    #[error("Internal server error")]
    InternalServerError(JsonValue),

    #[error("Email Already exists")]
    EmailAlreadyExists,

    #[error("Failed to send confirmation email")]
    EmailSend(JsonValue),

    #[error("OCR Service Unavailable")]
    OcrServiceError,

    #[error("Database error")]
    DatabaseError(JsonValue),
}

/* -------------------------------------------------------------------------- */
/*                         ACTIX RESPONSE IMPLEMENTATION                      */
/* -------------------------------------------------------------------------- */

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::Forbidden(_) => StatusCode::FORBIDDEN,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EmailAlreadyExists => StatusCode::CONFLICT,
            Error::EmailSend(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::OcrServiceError => StatusCode::SERVICE_UNAVAILABLE,
            Error::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_logger = ErrorLogger::from_env();

        let error_logger = ErrorLogger::from_env();

        match self {
            Error::InternalServerError(context) => {
                let error_log = ErrorLog::new(
                    "InternalServerError",
                    context.get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Internal server error")
                ).with_stack_trace(
                    context.get("stack_trace")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string()
                ).with_metadata(
                    "context".to_string(),
                    context.to_string()
                );

                let error_id = error_logger.log(&error_log);
                log::error!("[{}] Internal error: {}", error_id, context);

                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal server error",
                    "error_id": error_id,
                    "message": "Check server logs with this error ID"
                }))
            },
            Error::BadRequest(v) => HttpResponse::build(self.status_code()).json(v),
            Error::Unauthorized(v) => HttpResponse::build(self.status_code()).json(v),
            Error::Forbidden(v) => HttpResponse::build(self.status_code()).json(v),
            Error::NotFound(v) => HttpResponse::build(self.status_code()).json(v),
            Error::UnprocessableEntity(v) => HttpResponse::build(self.status_code()).json(v),
            Error::EmailAlreadyExists => {
                let error_log = ErrorLog::new(
                    "EmailAlreadyExists",
                    "Attempted to register with an email that already exists in the system"
                );
                let error_id = error_logger.log(&error_log);

                HttpResponse::Conflict().json(json!({
                    "error": "Email already exists",
                    "error_id": error_id
                }))
            },
            Error::EmailSend(v) => {
                let error_log = ErrorLog::new(
                    "EmailSendError",
                    format!("Failed to send email: {}", v)
                );
                let error_id = error_logger.log(&error_log);

                HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to send email",
                    "error_id": error_id,
                    "details": v
                }))
            },
            Error::OcrServiceError => {
                let error_log = ErrorLog::new(
                    "OcrServiceError",
                    "OCR service (Tesseract) is unreachable or failed to process image"
                );
                let error_id = error_logger.log(&error_log);

                HttpResponse::ServiceUnavailable().json(json!({
                    "error": "OCR service is currently unavailable",
                    "error_id": error_id,
                    "details": "Check if the tesseract container is running"
                }))
            },
            Error::DatabaseError(v) => {
                let error_log = ErrorLog::new(
                    "DatabaseError",
                    format!("Database operation failed: {}", v)
                ).with_metadata(
                    "error_details".to_string(),
                    v.to_string()
                );
                let error_id = error_logger.log(&error_log);

                HttpResponse::InternalServerError().json(json!({
                    "error": "Database error",
                    "error_id": error_id,
                    "message": "Check server logs with this error ID"
                }))
            },
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                               CONVERSIONS                                  */
/* -------------------------------------------------------------------------- */

/* ----- JSON ----- */

impl From<JsonPayloadError> for Error {
    fn from(err: JsonPayloadError) -> Self {
        Error::UnprocessableEntity(json!({
            "error": "Invalid JSON payload",
            "details": err.to_string()
        }))
    }
}

/* ----- QUERY ----- */

impl From<QueryPayloadError> for Error {
    fn from(err: QueryPayloadError) -> Self {
        Error::UnprocessableEntity(json!({
            "error": "Invalid query parameters",
            "details": err.to_string()
        }))
    }
}

/* ----- RAW PAYLOAD ----- */

impl From<PayloadError> for Error {
    fn from(err: PayloadError) -> Self {
        Error::BadRequest(json!({
            "error": "Invalid payload",
            "details": err.to_string()
        }))
    }
}

/* ----- MAILBOX ----- */

impl From<MailboxError> for Error {
    fn from(err: MailboxError) -> Self {
        Error::InternalServerError(json!({
            "message": "Mailbox error",
            "error": err.to_string()
        }))
    }
}

/* ----- JWT ----- */

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        match error.kind() {
            JwtErrorKind::InvalidToken => Error::Unauthorized(json!({
                "error": "Invalid token"
            })),
            JwtErrorKind::InvalidIssuer => Error::Unauthorized(json!({
                "error": "Invalid token issuer"
            })),
            _ => Error::Unauthorized(json!({
                "error": "Token authentication failed"
            })),
        }
    }
}

/* ----- SEA ORM ----- */

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => Error::NotFound(json!({
                "error": "Record not found"
            })),
            DbErr::Exec(e) => {
                Error::InternalServerError(json!({
                    "message": "Database execution failed",
                    "error": e.to_string(),
                    "error_type": "DbExec"
                }))
            },
            DbErr::Query(e) => {
                Error::InternalServerError(json!({
                    "message": "Database query failed",
                    "error": e.to_string(),
                    "error_type": "DbQuery"
                }))
            },
            _ => Error::InternalServerError(json!({
                "message": "Unexpected database error",
                "error": err.to_string(),
                "error_type": "DbUnknown"
            })),
        }
    }
}

impl From<TransactionError<Error>> for Error {
    fn from(err: TransactionError<Error>) -> Self {
        match err {
            TransactionError::Connection(e) => Error::from(e), // Uses your existing From<DbErr>
            TransactionError::Transaction(e) => e,             // Your app error
        }
    }
}

/* ----- VALIDATOR ----- */

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut map = JsonMap::new();

        for (field, errs) in errors.field_errors() {
            let messages: Vec<JsonValue> = errs.iter().map(|e| json!(e.message)).collect();

            map.insert(field.to_string(), json!(messages));
        }

        Error::UnprocessableEntity(json!({
            "errors": map
        }))
    }
}

/* ----- fs ----- */

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => Error::NotFound(json!({
                "error": "File or directory not found",
                "details": err.to_string()
            })),
            io::ErrorKind::PermissionDenied => Error::Forbidden(json!({
                "error": "Storage permission denied",
                "details": err.to_string()
            })),
            _ => {
                Error::InternalServerError(json!({
                    "message": "Filesystem operation failed",
                    "error": err.to_string(),
                    "error_kind": format!("{:?}", err.kind())
                }))
            }
        }
    }
}

/* ----- serde ----- */

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        if err.is_data() || err.is_syntax() {
            Error::BadRequest(json!({
                "error": "Malformed JSON data",
                "detail": err.to_string()
            }))
        } else {
            Error::InternalServerError(json!({
                "message": "JSON serialization error",
                "error": err.to_string()
            }))
        }
    }
}

/* ----- Uuid ----- */

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Error::BadRequest(format!("Invalid UUID: {}", err).into())
    }
}

/* ----- reqwest ----- */
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() || err.is_connect() {
            Error::OcrServiceError
        } else if err.is_decode() {
            Error::BadRequest(json!({
                "error": "OCR service returned unreadable data",
                "details": err.to_string()
            }))
        } else {
            Error::InternalServerError(json!({
                "message": "HTTP request failed",
                "error": err.to_string(),
                "is_timeout": err.is_timeout(),
                "is_connect": err.is_connect()
            }))
        }
    }
}

/* ----- Hasher ----- */

impl From<libreauth::pass::Error> for Error {
    fn from(err: libreauth::pass::Error) -> Self {
        Error::UnprocessableEntity(json!({
            "error": format!("Password hashing error: {}", err)
        }))
    }
}



impl From<actix_web::Error> for Error {
    fn from(err: actix_web::Error) -> Self {
        // This catches multipart errors, file size limit errors, etc.
        Error::BadRequest(json!({
            "error": "Request error",
            "details": err.to_string()
        }))
    }
}
/* ----- SQLX ----- */
impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound(json!({
                "error": "Resource not found in dictionary"
            })),
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    Error::BadRequest(json!({
                        "error": "This entry already exists in the lexicon"
                    }))
                } else {
                    Error::DatabaseError(json!({
                        "error": "Database constraint violation",
                        "details": db_err.message()
                    }))
                }
            }
            _ => Error::InternalServerError(json!({
                "message": "SQLx database error",
                "error": err.to_string(),
                "error_type": format!("{:?}", err)
            })),
        }
    }
}


        /* -------------------------------------------------------------------------- */
/*                        ACTIX CONFIG ERROR HANDLERS                         */
/* -------------------------------------------------------------------------- */

pub fn json_error_handler(
    err: JsonPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let app_error: Error = err.into();
    actix_web::Error::from(app_error)
}

pub fn query_error_handler(
    err: QueryPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let app_error: Error = err.into();
    actix_web::Error::from(app_error)
}
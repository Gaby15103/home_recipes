use actix::MailboxError;
use actix_web::{
    error::{QueryPayloadError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DieselError},
};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use libreauth::pass::ErrorCode as PassErrorCode;
use serde_json::{json, Map as JsonMap, Value as JsonValue};
use thiserror::Error;
use validator::ValidationErrors;

/// Application-level error type
#[derive(Debug, Error)]
pub enum Error {
    // 400
    #[error("Bad request")]
    BadRequest(JsonValue),

    // 401
    #[error("Unauthorized")]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden")]
    Forbidden(JsonValue),

    // 404
    #[error("Not found")]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable entity")]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal server error")]
    InternalServerError,

    #[error("Failed to send confirmation email")]
    EmailSend(JsonValue),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::Forbidden(_) => StatusCode::FORBIDDEN,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EmailSend(_) =>  StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal server error"
                }))
            }
            Error::EmailSend(v) => {
                HttpResponse::InternalServerError().json(v)
            }
            Error::BadRequest(v)
            | Error::Unauthorized(v)
            | Error::Forbidden(v)
            | Error::NotFound(v)
            | Error::UnprocessableEntity(v) => {
                HttpResponse::build(self.status_code()).json(v)
            }
        }
    }
}

/* -------------------- Conversions -------------------- */

impl From<QueryPayloadError> for Error {
    fn from(err: QueryPayloadError) -> Self {
        Error::UnprocessableEntity(json!({
            "error": "Invalid query parameters",
            "details": err.to_string(),
        }))
    }
}

impl From<MailboxError> for Error {
    fn from(_: MailboxError) -> Self {
        Error::InternalServerError
    }
}

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

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => Error::NotFound(json!({
                "error": "Requested record not found"
            })),
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                Error::UnprocessableEntity(json!({
                    "error": info
                        .details()
                        .unwrap_or_else(|| info.message())
                }))
            }
            _ => Error::InternalServerError,
        }
    }
}

impl From<PoolError> for Error {
    fn from(_: PoolError) -> Self {
        Error::InternalServerError
    }
}

impl From<PassErrorCode> for Error {
    fn from(_: PassErrorCode) -> Self {
        Error::InternalServerError
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut map = JsonMap::new();

        for (field, errs) in errors.field_errors() {
            let messages: Vec<JsonValue> = errs
                .iter()
                .map(|e| json!(e.message))
                .collect();

            map.insert(field.to_string(), json!(messages));
        }

        Error::UnprocessableEntity(json!({
            "errors": map
        }))
    }
}

impl From<libreauth::pass::Error> for Error {
    fn from(err: libreauth::pass::Error) -> Self {
        Error::UnprocessableEntity(json!({
            "error": format!("Password hashing error: {}", err)
        }))
    }
}

/* -------------------- DB Error Layer -------------------- */

#[derive(Debug)]
pub enum DbError {
    NotFound,
    Forbidden,
    Diesel(DieselError),
    Pool(r2d2::Error),
}

impl From<DieselError> for DbError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => DbError::NotFound,
            e => DbError::Diesel(e),
        }
    }
}

impl From<r2d2::Error> for DbError {
    fn from(err: r2d2::Error) -> Self {
        DbError::Pool(err)
    }
}

impl From<DbError> for Error {
    fn from(err: DbError) -> Self {
        match err {
            DbError::NotFound => Error::NotFound(json!({
                "error": "Record not found"
            })),
            DbError::Forbidden => Error::Forbidden(json!({
                "error": "Forbidden"
            })),
            DbError::Diesel(_) | DbError::Pool(_) => Error::InternalServerError,
        }
    }
}

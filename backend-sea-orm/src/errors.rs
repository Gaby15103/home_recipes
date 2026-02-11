use actix::MailboxError;
use actix_web::{
    HttpResponse,
    error::{JsonPayloadError, PayloadError, QueryPayloadError, ResponseError},
    http::StatusCode,
};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use sea_orm::{DbErr, TransactionError};
use serde_json::{Map as JsonMap, Value as JsonValue, json};
use thiserror::Error;
use validator::ValidationErrors;

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
    InternalServerError,

    #[error("Email Already exists")]
    EmailAlreadyExists,

    #[error("Failed to send confirmation email")]
    EmailSend(JsonValue),
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
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EmailAlreadyExists => StatusCode::CONFLICT,
            Error::EmailSend(_) =>  StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => HttpResponse::InternalServerError().json(json!({
                "error": "Internal server error"
            })),
            | Error::BadRequest(v)
            | Error::Unauthorized(v)
            | Error::Forbidden(v)
            | Error::NotFound(v)
            | Error::UnprocessableEntity(v) => HttpResponse::build(self.status_code()).json(v),
            | Error::EmailAlreadyExists => HttpResponse::Conflict().json(json!({
                "error": "Email already exists"
            })),
            | Error::EmailSend(v) => {
                HttpResponse::InternalServerError().json(v)
            }
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
    fn from(_: MailboxError) -> Self {
        Error::InternalServerError
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
            DbErr::Exec(_) | DbErr::Query(_) => Error::InternalServerError,
            _ => Error::InternalServerError,
        }
    }
}

impl From<TransactionError<Error>> for Error {
    fn from(err: TransactionError<Error>) -> Self {
        match err {
            TransactionError::Connection(e) => Error::from(e), // Uses your existing From<DbErr>
            TransactionError::Transaction(e) => e,            // Your app error
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

/* ----- Hasher ----- */

impl From<libreauth::pass::Error> for Error {
    fn from(err: libreauth::pass::Error) -> Self {
        Error::UnprocessableEntity(json!({
            "error": format!("Password hashing error: {}", err)
        }))
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

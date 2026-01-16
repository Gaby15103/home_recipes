use actix::MailboxError;
use actix_web::error::QueryPayloadError;
use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DieselError},
};
use jwt::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use libreauth::pass::ErrorCode as PassErrorCode;
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::convert::From;
use validator::ValidationErrors;

#[derive(Fail, Debug)]
pub enum Error {
    // 401
    #[fail(display = "Unauthorized: {}", _0)]
    Unauthorized(JsonValue),

    // 403
    #[fail(display = "Forbidden: {}", _0)]
    Forbidden(JsonValue),

    // 404
    #[fail(display = "Not Found: {}", _0)]
    NotFound(JsonValue),

    // 422
    #[fail(display = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    // 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,
}

// the ResponseError trait lets us convert errors to http responses with appropriate data
// https://actix.rs/docs/errors/

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            Error::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            Error::NotFound(ref message) => HttpResponse::NotFound().json(message),
            Error::UnprocessableEntity(ref message) => {
                HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message)
            }
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }
}

impl From<QueryPayloadError> for Error {
    fn from(err: QueryPayloadError) -> Self {
        Error::UnprocessableEntity(json!({
            "error": "Invalid query parameters",
            "details": err.to_string(),
        }))
    }
}

impl From<MailboxError> for Error {
    fn from(err: MailboxError) -> Self {
        Error::InternalServerError
    }
}

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        match error.kind() {
            JwtErrorKind::InvalidToken => Error::Unauthorized(json!({
                "error": "Token is invalid",
            })),
            JwtErrorKind::InvalidIssuer => Error::Unauthorized(json!({
                "error": "Issuer is invalid",
            })),
            _ => Error::Unauthorized(json!({
                "error": "An issue was found with the token provided",
            })),
        }
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::UnprocessableEntity(json!({ "error": message }));
                }
                Error::InternalServerError
            }
            DieselError::NotFound => {
                Error::NotFound(json!({ "error": "requested record was not found" }))
            }
            _ => Error::InternalServerError,
        }
    }
}

impl From<PoolError> for Error {
    fn from(_error: PoolError) -> Self {
        Error::InternalServerError
    }
}

impl From<PassErrorCode> for Error {
    fn from(_error: PassErrorCode) -> Self {
        Error::InternalServerError
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        // transforms errors into objects that err_map can take
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors
                .iter()
                .map(|error| {
                    // dbg!(error) // <- Uncomment this if you want to see what error looks like
                    json!(error.message)
                })
                .collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        Error::UnprocessableEntity(json!({
            "errors": err_map,
        }))
    }
}

impl From<libreauth::pass::Error> for Error {
    fn from(err: libreauth::pass::Error) -> Self {
        Error::UnprocessableEntity(json!({
            "error": format!("Password hash error: {}", err),
        }))
    }
}

#[derive(Debug)]
pub enum DbError {
    NotFound,
    Forbidden,
    Diesel(diesel::result::Error),
    Pool(r2d2::Error),
}
impl From<diesel::result::Error> for DbError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => DbError::NotFound,
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
    fn from(e: DbError) -> Self {
        match e {
            DbError::NotFound => Error::NotFound(json!({"error": "Record not found"})),
            DbError::Forbidden => Error::Forbidden(json!({"error": "Forbidden"})),
            DbError::Diesel(_) | DbError::Pool(_) => Error::InternalServerError,
        }
    }
}

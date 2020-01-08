use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as DBError};
use juniper::graphql_value;
use std::convert::From;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unable to connect to DB")]
    UnableToConnectToDb,
}

impl juniper::IntoFieldError for ServiceError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            ServiceError::Unauthorized => juniper::FieldError::new(
                "Unauthorized",
                graphql_value!({
                    "type": "NO_ACCESS"
                }),
            ),

            _ => juniper::FieldError::new(
                "Unknown Error",
                graphql_value!({
                    "type": "UNKNOWN_ERROR"
                }),
            ),
        }
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::UnableToConnectToDb => HttpResponse::InternalServerError()
                .json("Unable to connect to DB, Please try later"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<uuid::parser::ParseError> for ServiceError {
    fn from(_: uuid::parser::ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;

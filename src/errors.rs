use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use juniper::graphql_value;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Duplicate")]
    Duplicate,
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

            ServiceError::Duplicate => juniper::FieldError::new(
                "User already exist",
                graphql_value!({
                    "type": "USER_EXIST"
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

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, _info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    return ServiceError::Duplicate;
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}
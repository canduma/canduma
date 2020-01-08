use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::model::Context;
use crate::jwt::model::Claims;

pub type ClaimsResponse = Claims;

#[juniper::object]
impl ClaimsResponse {
    fn iss(&self) -> &str {
        self.iss.as_str()
    }
    fn email(&self) -> &str {
        self.email.as_str()
    }
    fn sub(&self) -> &str {
        self.sub.as_str()
    }
    fn iat(&self) -> String {
        chrono::NaiveDateTime::from_timestamp(self.iat, 0)
            .format("%Y-%m-%dT%H:%M:%S%.f")
            .to_string()
    }
    fn exp(&self) -> String {
        chrono::NaiveDateTime::from_timestamp(self.exp, 0)
            .format("%Y-%m-%dT%H:%M:%S%.f")
            .to_string()
    }
}

pub(crate) fn decode(context: &Context) -> ServiceResult<&ClaimsResponse> {
    match context.token.jwt {
        None => Err(ServiceError::Unauthorized),
        Some(ref m) => Ok(m as &ClaimsResponse),
    }
}

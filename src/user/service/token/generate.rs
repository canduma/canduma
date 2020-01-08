use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::model::Context;
use crate::jwt::manager::create_token;
use crate::jwt::model::Token;

pub(crate) fn generate(context: &Context) -> ServiceResult<Token> {
    match context.user.0 {
        None => Err(ServiceError::Unauthorized),
        Some(ref user) => {
            match create_token(
                user,
                context.opt.domain.clone(),
                context.opt.auth_duration_in_hour,
            ) {
                Ok(r) => Ok(Token { bearer: Some(r) }),
                Err(e) => Err(e),
            }
        }
    }
}

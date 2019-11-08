use crate::jwt::manager::decode_token;
use crate::jwt::model::{Claims, DecodedToken};
use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;

impl FromRequest for DecodedToken {
    type Error = Error;
    type Future = Result<DecodedToken, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        let token = match BearerAuth::extract(req) {
            Ok(t) => Some(t.token().to_string()),
            Err(_) => None,
        };

        // TODO: Send a Result<LoggedUser, ServiceError> to Context
        match token {
            None => return Ok(DecodedToken { jwt: None }),
            Some(token) => match decode_token(&token) {
                Ok(decoded) => {
                    return Ok(DecodedToken {
                        jwt: Some(decoded as Claims),
                    })
                }
                Err(_) => return Ok(DecodedToken { jwt: None }),
            },
        }
    }
}

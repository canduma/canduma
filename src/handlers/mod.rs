use actix_web::{FromRequest, HttpResponse, HttpRequest, dev};
use crate::utils::jwt::decode_token;
use crate::models::user::SlimUser;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = HttpResponse;
    type Future = Result<Self, HttpResponse>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {

        let token = match BearerAuth::extract(req) {
            Ok(t) => Some(t.token().to_string()),
            Err(_) => None
        };

        // TODO: Send a Result<LoggedUser, ServiceError> to Context
        match token {
            None => return Ok(LoggedUser { email: None }),
            Some(token) => {
                match decode_token(&token) {
                    Ok(decoded) => Ok(decoded as LoggedUser),
                    Err(_) => return Ok(LoggedUser { email: None }),
                }
            }
        }
    }
}
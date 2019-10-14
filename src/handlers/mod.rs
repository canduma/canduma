use actix_web::{FromRequest, HttpResponse, HttpRequest, dev};
use crate::utils::jwt::decode_token;
use crate::models::user::SlimUser;

pub type LoggedUser = SlimUser;


/// Extract a new `LoggedUser` from Request.
/// todo: Verify expiration of token and info and handle error into ServiceError
impl FromRequest for LoggedUser {
    type Error = HttpResponse;
    type Future = Result<Self, HttpResponse>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let token =
            req
                .headers()
                .get("x-auth-token");

        match token {
            None => return Ok(LoggedUser { email: "".to_string() }),
            Some(token) => {
                let user: SlimUser = decode_token(&token.to_str().unwrap())?;
                return Ok(user as LoggedUser);
            }
        }
        Ok(LoggedUser { email: "".to_string() })
    }
}
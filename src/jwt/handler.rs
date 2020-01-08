use crate::jwt::manager::decode_token;
use crate::jwt::model::DecodedToken;
use actix_web::dev::Payload;
use actix_web::{http::header, Error, FromRequest, HttpRequest};
use regex::Regex;

lazy_static::lazy_static! {
    static ref BEARER_REGEXP : Regex = Regex::new(r"^Bearer\s(.*)$").expect("Bearer regexp failed!");
}

impl FromRequest for DecodedToken {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|authorization| {
                BEARER_REGEXP
                    .captures(authorization)
                    .and_then(|captures| captures.get(1))
            })
            .map(|v| v.as_str());

        futures::future::ready(Ok(match token {
            None => DecodedToken { jwt: None },
            Some(token) => match decode_token(token) {
                Ok(decoded) => DecodedToken { jwt: Some(decoded) },
                Err(_) => DecodedToken { jwt: None },
            },
        }))
    }
}

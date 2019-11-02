use crate::models::user::SlimUser;
use std::convert::From;
use jsonwebtoken::{decode, encode, Header, Validation, Algorithm};
use chrono::{Local, Duration};
use dotenv::dotenv;
use std::{env, fs};
use actix_web::HttpResponse;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // issuer
    iss: String,
    // subject
    sub: String,
    //issued at
    iat: i64,
    // expiry
    exp: i64,
    // user email
    email: String,
}

// struct to get converted to token and back
impl Claims {
    fn with_email(email: &str) -> Self {
        dotenv().ok();
        Claims {
            iss: env::var("DOMAIN").unwrap_or("localhost".into()),
            sub: "auth".into(),
            email: email.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser { email: Some(claims.email) }
    }
}

pub fn create_token(email: &str) -> Result<String, HttpResponse> {
    let claims = Claims::with_email(email);
    encode(&Header::new(Algorithm::RS256), &claims, include_bytes!("../../keys/private_rsa_key.der"))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<SlimUser, HttpResponse> {
    decode::<Claims>(token, include_bytes!("../../keys/public_rsa_key.der"), &Validation::new(Algorithm::RS256))
        .map(|data| data.claims.into())
        .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}

/*
// take a string from env variable
fn get_secret() -> String {
    //dotenv().ok();
    //env::var("SECRET").unwrap_or("my secret".into())
    fs::read_to_string("./keys/private.der")
        .expect("Something went wrong reading the file")
}*/

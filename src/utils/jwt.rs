use crate::models::user::SlimUser;
use std::convert::From;
use jsonwebtoken::{decode, encode, Header, Validation} ;
use chrono::{Local, Duration};
use dotenv::dotenv;
use std::env;
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
        Claims {
            iss: "localhost".into(),
            sub: "auth".into(),
            email: email.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser { email: claims.email }
    }
}

pub fn create_token(email: &str) -> Result<String, HttpResponse> {
    let claims = Claims::with_email(email);
    encode(&Header::default(), &claims, get_secret().as_ref())
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<SlimUser, HttpResponse> {
    decode::<Claims>(token, get_secret().as_ref(), &Validation::default())
        .map(|data| data.claims.into())
        .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}

// take a string from env variable
fn get_secret() -> String {
    dotenv().ok();
    env::var("SECRET").unwrap_or("my secret".into())
}
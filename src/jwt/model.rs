use crate::user::model::SlimUser;
use chrono::format::StrftimeItems;
use chrono::{Duration, Local, NaiveDateTime};
use dotenv::dotenv;
use std::convert::From;
use std::env;

#[derive(Clone)]
pub struct DecodedToken {
    pub jwt: Option<Claims>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // issuer
    pub iss: String,
    // subject
    pub sub: String,
    //issued at
    pub iat: i64,
    // expiry
    pub exp: i64,
    // user email
    pub email: String,
}

pub type HumanClaims = Claims;

#[juniper::object]
impl HumanClaims {
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
        let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
        NaiveDateTime::from_timestamp(*&self.iat, 0)
            .format_with_items(fmt.clone())
            .to_string()
    }
    fn exp(&self) -> String {
        let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
        NaiveDateTime::from_timestamp(*&self.exp, 0)
            .format_with_items(fmt.clone())
            .to_string()
    }
}
// struct to get converted to token and back
impl Claims {
    pub fn with_email(email: &str) -> Self {
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

#[derive(juniper::GraphQLObject)]
pub struct Token {
    pub bearer: Option<String>,
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            email: Some(claims.email),
        }
    }
}

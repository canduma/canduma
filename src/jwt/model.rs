use crate::user::model::SlimUser;
use anyhow::Result;
use chrono::{Duration, Local};
use std::convert::TryFrom;
use uuid::Uuid;

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
    // issued at
    pub iat: i64,
    // expiry
    pub exp: i64,
    // user email
    pub email: String,
    // user role
    pub role: String,
}

// struct to get converted to token and back
impl Claims {
    pub(crate) fn new(slim_user: &SlimUser, issuer: String, auth_duration_in_hour: u16) -> Self {
        let SlimUser {
            email,
            user_uuid,
            role,
            ..
        } = slim_user;

        let iat = Local::now();
        let exp = iat + Duration::hours(i64::from(auth_duration_in_hour));

        Claims {
            iss: issuer,
            sub: user_uuid.to_string(),
            email: email.clone(),
            role: role.clone(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Token {
    pub bearer: Option<String>,
}

impl TryFrom<Claims> for SlimUser {
    type Error = anyhow::Error;

    fn try_from(claims: Claims) -> Result<Self> {
        let Claims {
            email, sub, role, ..
        } = claims;

        Ok(SlimUser {
            email,
            user_uuid: Uuid::parse_str(&sub)?,
            role,
        })
    }
}

use crate::errors::ServiceError;
use crate::jwt::model::Claims;
use crate::user::model::SlimUser;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};

pub fn create_token(
    user: &SlimUser,
    issuer: String,
    auth_duration_in_hour: u16,
) -> Result<String, ServiceError> {
    let claims: Claims = Claims::new(user, issuer, auth_duration_in_hour);

    encode(
        &Header::new(Algorithm::RS256),
        &claims,
        include_bytes!("../../keys/rs256-4096-private.rsa"),
    )
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<Claims, ServiceError> {
    decode::<Claims>(
        token,
        include_bytes!("../../keys/rs256-4096-public.pem"),
        &Validation::new(Algorithm::RS256),
    )
    .map(|data| data.claims)
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

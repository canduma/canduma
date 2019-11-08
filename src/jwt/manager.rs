use crate::errors::ServiceError;
use crate::jwt::model::Claims;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};

pub fn create_token(email: &str) -> Result<String, ServiceError> {
    let claims = Claims::with_email(email);
    encode(
        &Header::new(Algorithm::RS256),
        &claims,
        include_bytes!("../../keys/private_rsa_key.der"),
    )
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<Claims, ServiceError> {
    decode::<Claims>(
        token,
        include_bytes!("../../keys/public_rsa_key.der"),
        &Validation::new(Algorithm::RS256),
    )
    .map(|data| data.claims)
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

use crate::errors::ServiceError;
use crate::jwt::model::Claims;
use crate::user::model::SlimUser;
use jsonwebtoken::{Algorithm, EncodingKey, DecodingKey, Header, Validation, decode, encode};

pub fn create_token(
    user: &SlimUser,
    issuer: String,
    auth_duration_in_hour: u16,
) -> Result<String, ServiceError> {
    let claims: Claims = Claims::new(user, issuer, auth_duration_in_hour);

    encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(include_bytes!("../../keys/rs256-4096-private.rsa")).unwrap(),
    )
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<Claims, ServiceError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_rsa_pem(include_bytes!("../../keys/rs256-4096-public.pem")).unwrap(),
        &Validation::new(Algorithm::RS256),
    )
    .map(|data| data.claims)
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

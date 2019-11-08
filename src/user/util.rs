use crate::errors::ServiceError;
use argon2rs::argon2i_simple;

pub fn make_salt() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 128;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

pub fn make_hash(password: &str, salt: &str) -> Vec<u8> {
    argon2i_simple(password, salt).to_vec()
}

pub fn verify(hash: &Vec<u8>, salt: &str, password: &str) -> Result<bool, ServiceError> {
    if &make_hash(password, salt) == hash {
        return Ok(true);
    }
    Err(ServiceError::Unauthorized)
}

lazy_static::lazy_static! {
pub  static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

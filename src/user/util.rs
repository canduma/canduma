use super::model::{LoggedUser, User};
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

pub fn make_hash(password: &str, salt: &str) -> [u8; argon2rs::defaults::LENGTH] {
    argon2i_simple(password, salt)
}

pub fn verify(user: &User, password: &str) -> bool {
    let User { hash, salt, .. } = user;

    make_hash(password, salt) == hash.as_ref()
}

pub fn has_role(user: &LoggedUser, role: &str) -> Result<bool, ServiceError> {
    match user.0 {
        Some(ref user) if user.role == role => Ok(true),
        _ => Err(ServiceError::Unauthorized),
    }
}

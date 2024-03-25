use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::RngCore;

use crate::error::{Error, Result};

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let hasher = Argon2::default();

    let hashed_password = hasher
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::Other("Failed to hash password".to_string()))?
        .to_string();

    Ok(hashed_password)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool> {
    let password_hash = PasswordHash::new(hashed_password)
        .map_err(|_| Error::Other("Invalid format of hash".to_string()))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok())
}

pub fn generate_token() -> String {
    (0..64)
        .map(|_| {
            let offset: u8 = (OsRng.next_u32() % 26) as u8;
            (97u8 + offset) as char
        })
        .collect::<String>()
}

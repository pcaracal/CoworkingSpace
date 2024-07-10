use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

#[allow(clippy::missing_panics_doc)]
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash).unwrap();
    argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok()
}

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

// Interface para criptografia de senha
pub trait PasswordEncryptorPort: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String, argon2::password_hash::Error>;
    fn verify_password(&self, hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error>;
}

// Implementação usando Argon2
#[derive(Clone)]
pub struct Argon2PasswordEncryptor;

impl Default for Argon2PasswordEncryptor {
    fn default() -> Self {
        Self::new()
    }
}

impl Argon2PasswordEncryptor {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordEncryptorPort for Argon2PasswordEncryptor {
    fn hash_password(&self, password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
    }

    fn verify_password(&self, hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

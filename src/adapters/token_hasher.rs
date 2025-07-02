use jsonwebtoken::{encode, EncodingKey, Header, errors::Error as JwtError};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::core::entities::auth::ClaimsToUserToken;


pub trait TokenGeneratorPort: Send + Sync {
    fn generate_token(&self, id: String, full_name: String, email: String, secret: &str) -> Result<String, JwtError>;
}

#[derive(Clone)]
pub struct JwtTokenGenerator;

impl JwtTokenGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl TokenGeneratorPort for JwtTokenGenerator {
    fn generate_token(&self, id: String, full_name: String, email: String, secret: &str) -> Result<String, JwtError> {
        let expiration: usize = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize + 24 * 3600;

        let claims = ClaimsToUserToken {
            id,
            exp: expiration,
            full_name,
            email,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
    }
}

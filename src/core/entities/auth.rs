use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsToUserToken {
    pub id: String,
    pub exp: usize,
    pub full_name: String,
    pub email: String,
}


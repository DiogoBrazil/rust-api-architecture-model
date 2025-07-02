use actix_web::{web, HttpResponse};
use log::info;
use crate::adapters::password_hasher::PasswordEncryptorPort;
use crate::adapters::token_hasher::TokenGeneratorPort;
use crate::config::config_env::Config;
use crate::core::contracts::repository::users::UserRepository;
use crate::core::entities::auth::{Login, LoginResponse};
use crate::repositories::user_repository::PgUserRepository;
use crate::utils::errors::AppError;
use crate::utils::response::ApiResponse;

pub struct AuthService {
    user_repo: web::Data<PgUserRepository>,
    config: web::Data<Config>,
    password_encryptor: Box<dyn PasswordEncryptorPort>,
    token_generator: Box<dyn TokenGeneratorPort>,
}

impl AuthService {
    pub fn new(
        user_repo: web::Data<PgUserRepository>,
        config: web::Data<Config>,
        password_encryptor: Box<dyn PasswordEncryptorPort>,
        token_generator: Box<dyn TokenGeneratorPort>,
    ) -> Self {
        Self { user_repo, config, password_encryptor, token_generator }
    }

    pub async fn login(&self, data: Login) -> Result<HttpResponse, AppError> {
        info!("[Service] Starting login process with email: {}", data.email);

        info!("[Service] Checking if user exists with email: {}", data.email);
        let user = match self.user_repo.find_user_by_email(data.email.clone()).await {
            Ok(user) => {
                info!("[Service] User found with email: {}", data.email);
                user
            },
            Err(sqlx::Error::RowNotFound) => {
                info!("[Service] User not found with email: {}", data.email);
                return Err(AppError::Unauthorized("Invalid credentials".into()));
            },
            Err(e) => {
                info!("[Service] Database error while finding user: {:?}", e);
                return Err(AppError::InternalServerError);
            }
        };

        info!("[Service] Verifying password for user with email: {}", data.email);
        if !self.password_encryptor.verify_password(&user.password, &data.password)
            .map_err(|_| AppError::InternalServerError)? {
            info!("[Service] Incorrect password for user with email: {}", data.email);
            return Err(AppError::Unauthorized("Invalid credentials".into()));
        }
        info!("[Service] Password verified successfully for user with email: {}", data.email);

        info!("[Service] Generating token for user with email: {}", data.email);
        let token = self.token_generator
            .generate_token(
                user.id.to_string(),
                user.full_name.clone(),
                user.email.to_string(),
                &self.config.jwt_secret,
            )
            .map_err(|_| AppError::InternalServerError)?;

        info!("[Service] Token generated successfully for user with email: {}", data.email);

        let response = LoginResponse {
            token,
            id: user.id,
            full_name: user.full_name,
            email: user.email
        };

        Ok(ApiResponse::success(response).into_response())
    }
}
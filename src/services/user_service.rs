use actix_web::{web, HttpResponse};
use log::{error, info};
use uuid::Uuid;
use crate::core::entities::user::{
    //ClaimsToUserToken,
    CreateUser,
    //UpdatePasswordUser,
    UpdateUser,
    UserDataCreated,
};
use crate::adapters::password_hasher::PasswordEncryptorPort;
use crate::core::contracts::repository::users::UserRepository;
use crate::repositories::user_repository::PgUserRepository;
use crate::utils::errors::AppError;
use crate::utils::response::ApiResponse;
use crate::utils::validations::{is_valid_email, validate_required_fields};
use crate::utils::{errors, response};


pub struct UserService {
    user_repo: web::Data<PgUserRepository>,
    password_encryptor: Box<dyn PasswordEncryptorPort>,
}

impl UserService {
    pub fn new(
        user_repo: web::Data<PgUserRepository>,
        password_encryptor: Box<dyn PasswordEncryptorPort>,
    ) -> Self {
        Self {user_repo, password_encryptor}
    }

    pub async fn create_user(&self, data: CreateUser) -> Result<HttpResponse, errors::AppError> {
        info!("[Service] Starting user creation process for email: {}", data.email);

        info!("[Service] Validating required fields");
        let fields = [
            ("full_name", data.full_name.is_empty()),
            ("email", data.email.is_empty()),
            ("password", data.password.is_empty()),
        ];

        validate_required_fields(&fields, "Error adding user")?;
        info!("[Service] Required fields validation passed");

        info!("[Service] Validating email format: {}", data.email);
        if !is_valid_email(&data.email) {
            return Err(errors::AppError::BadRequest(
                format!("Error adding user: '{}' is not a valid email", data.email)
            ));
        }
        info!("[Service] Email format validation passed");

        info!("[Service] Checking if user already exists with email: {}", data.email);
        if let Ok(_existing_user) = self.user_repo.find_user_by_email(data.email.clone()).await {
            info!("[Service] User already exists with email: {}", data.email);
            return Err(errors::AppError::BadRequest(
                format!("Error adding user: email '{}' already exists", data.email)
            ));
        }
        info!("[Service] User does not exist, proceeding with creation");

        info!("[Service] Hashing user password");
        let mut user_with_hash = data;
        user_with_hash.password = self.password_encryptor
            .hash_password(&user_with_hash.password)
            .map_err(|e| {
                error!("[Service] Error hashing password: {:?}", e);
                errors::AppError::InternalServerError
            })?;
        info!("[Service] Password hashed successfully");

        info!("[Service] Saving user to database");
        match self.user_repo.create_user(user_with_hash).await {
            Ok(user) => {
                info!("[Service] User created successfully with ID: {}", user.id);
                Ok(response::ApiResponse::created(UserDataCreated::from(user)).into_response())
            },
            Err(e) => {
                error!("[Service] Error creating user in database: {:?}", e);
                Err(errors::AppError::InternalServerError)
            }
        }
    }

    pub async fn update_user(&self, data: UpdateUser, id: Uuid) -> Result<HttpResponse, AppError> {
        info!("[Service] Starting update user process for email: {}", data.email.clone());

        info!("[Service] Checking if user exists with id: {}", id);
        match self.user_repo.find_user_by_id(id).await {
            Ok(_existing_user) => {
                info!("[Service] User exists, proceeding with update");
            }
            Err(_) => {
                info!("[Service] User not found with id: {}", id);
                return Err(AppError::BadRequest(
                    format!("Error updating user: id '{}' not found", id)
                ));
            }
        }

        info!("[Service] Validating required fields");
        let fields = [
            ("full_name", data.full_name.is_empty()),
            ("email", data.email.is_empty()),
        ];

        validate_required_fields(&fields, "Error updating user")?;
        info!("[Service] Required fields validation passed");

        info!("[Service] Validating email format: {}", data.email);
        if !is_valid_email(&data.email) {
            return Err(errors::AppError::BadRequest(
                format!("Error updating user: '{}' is not a valid email", data.email)
            ));
        }
        info!("[Service] Email format validation passed");

        info!("[Service] Checking if the email is already in use by another user");
        if self.user_repo.email_exists_for_other_user(&data.email, id).await? {
            return Err(AppError::BadRequest(
                format!("Email '{}' is already in use by another user", data.email)
            ));
        }
        info!("[Service] There is no user with the email {} registered in the database, proceeding with the update", data.email);

        info!("[Service] Saving user to database");
        match self.user_repo.update_user(id, data).await {
            Ok(user) => {
                info!("[Service] User updated successfully with ID: {}", user.id);
                Ok(response::ApiResponse::updated(UserDataCreated::from(user)).into_response())
            },
            Err(e) => {
                error!("[Service] Error updating user in database: {:?}", e);
                Err(errors::AppError::InternalServerError)
            }
        }
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> Result<HttpResponse, AppError> {
        info!("[Service] Starting find user by id process for id: {}", id);

        match self.user_repo.find_user_by_id(id).await {
            Ok(user) => {
                info!("[Service] User with id {} found successfully", user.id);
                Ok(response::ApiResponse::success(user).into_response())
            }
            Err(sqlx::Error::RowNotFound) => {
                info!("[Service] User with id {} not found", id);
                Err(AppError::NotFound(format!("User with id '{}' not found", id)))
            }
            Err(e) => {
                error!("[Service] Database error while finding user: {:?}", e);
                Err(AppError::InternalServerError)
            }
        }
    }

    pub async fn delete_user_by_id(&self, id: Uuid) -> Result<HttpResponse, AppError> {
        info!("[Service] Starting delete user by id process for id: {}", id);

        info!("[Service] Checking if user exists with id: {}", id);
        match self.user_repo.find_user_by_id(id).await {
            Ok(_existing_user) => {
                info!("[Service] User exists, proceeding with delete");
            }
            Err(_) => {
                info!("[Service] User not found with id: {}", id);
                return Err(AppError::BadRequest(
                    format!("Error deleting user: id '{}' not found", id)
                ));
            }
        }

        info!("[Service] Deleting user to database");
        match self.user_repo.delete_user(id).await {
            Ok(true) => {
                info!("[Service] User deleted successfully with ID: {}", id);
                Ok(ApiResponse::success(()).into_response())
            }
            Ok(false) =>{
                error!("[Service] Database error while deleting user");
                Err(AppError::NotFound(format!("User '{}' not found", id)))
            }
            Err(e) => {
                error!("[Service] Database error while deleting user: {:?}", e);
                Err(AppError::InternalServerError)
            }
        }
    }

    pub async fn find_all_users(&self) -> Result<HttpResponse, AppError> {
        info!("[Service] Starting find users process");

        match self.user_repo.find_all_users().await {
            Ok(users) => {
                info!("[Service] Found {} users", users.len());
                Ok(ApiResponse::success(users).into_response())
            }
            Err(e) => {
                error!("[Service] Database error while finding users: {:?}", e);
                Err(AppError::InternalServerError)
            }
        }
    }
}

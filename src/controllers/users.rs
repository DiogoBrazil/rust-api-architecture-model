use actix_web::{web, HttpResponse};
use log::info;
use uuid::Uuid;
use crate::services::user_service::UserService;
use crate::core::entities::user::{CreateUser, UpdateUser};
use crate::utils::errors::AppError;

pub async fn create_user(data: web::Json<CreateUser>, service: web::Data<UserService>) -> Result<HttpResponse, AppError> {
    info!("[Controller] Received request to create user with email: {}", data.email);
    let result = service.create_user(data.into_inner()).await;
    match &result {
        Ok(_) => info!("[Controller] User creation request completed successfully"),
        Err(e) => info!("[Controller] User creation request failed: {:?}", e)
    }
    result
}

pub async fn update_user(data: web::Json<UpdateUser>, id: web::Path<Uuid>, service: web::Data<UserService>) -> Result<HttpResponse, AppError> {
    info!("[Controller] Received request to update user with email: {}", data.email);
    let result = service.update_user(data.into_inner(), id.into_inner()).await;
    match &result {
        Ok(_) => info!("[Controller] User updated request completed successfully"),
        Err(e) => info!("[Controller] User updated request failed: {:?}", e)
    }
    result
}

pub async fn find_user_by_id(id: web::Path<Uuid>, service: web::Data<UserService>) -> Result<HttpResponse, AppError> {
    info!("[Controller] Received request to find user by id with id: {}", id);
    let result = service.find_user_by_id(id.into_inner()).await;
    match &result {
        Ok(_) => info!("[Controller] Find user by id request completed successfully"),
        Err(e) => info!("[Controller] Find user by id request failed: {:?}", e)
    }
    result
}

pub async fn delete_user_by_id(id: web::Path<Uuid>, service: web::Data<UserService>) -> Result<HttpResponse, AppError> {
    info!("[Controller] Received request to delete user by id with id: {}", id);
    let result = service.delete_user_by_id(id.into_inner()).await;
    match &result {
        Ok(_) => info!("[Controller] Delete user by id request completed successfully"),
        Err(e) => info!("[Controller] Delete user by id request failed: {:?}", e)
    }
    result
}

pub async fn find_all_users(service: web::Data<UserService>) -> Result<HttpResponse, AppError> {
    info!("[Controller] Received request to finda all users");
    let result = service.find_all_users().await;
    match &result {
        Ok(_) => info!("[Controller] Find all users request completed successfully"),
        Err(e) => info!("[Controller] Find all users request failed: {:?}", e)
    }
    result
}

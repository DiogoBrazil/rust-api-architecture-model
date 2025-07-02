use actix_web::{web, HttpResponse};
use log::info;
use crate::services::auth_service::AuthService;
use crate::core::entities::auth::Login;
use crate::utils::errors::AppError;


pub async fn login(data: web::Json<Login>, service: web::Data<AuthService>) -> Result<HttpResponse, AppError> {
    info!("[Controller] Received request to login user with email: {}", data.email);
    let result = service.login(data.into_inner()).await;
    match &result { 
        Ok(_) => info!("[Controller] Login request completed successfully"),
        Err(e) =>  info!("[Controller] User creation request failed: {:?}", e)
    }
    result
}
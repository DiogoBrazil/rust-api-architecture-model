use actix_web::{error::ResponseError, HttpResponse, http::StatusCode};
use log::error;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Database Error: {0}")]
    DatabaseError(String),

    #[error("Invalid Method Error: {0}")]
    InvalidMethodError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        error!("Error occurred: {}", self);
        let (status_code, error_type) = match self {
            AppError::InternalServerError =>
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            AppError::BadRequest(_) =>
                (StatusCode::BAD_REQUEST, "Bad Request"),
            AppError::Unauthorized(_) =>
                (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::Forbidden(_) =>
                (StatusCode::FORBIDDEN, "Forbidden"),
            AppError::NotFound(_) =>
                (StatusCode::NOT_FOUND, "Not Found"),
            AppError::DatabaseError(_) =>
                (StatusCode::INTERNAL_SERVER_ERROR, "Database Error"),
            AppError::InvalidMethodError(_) =>
                (StatusCode::METHOD_NOT_ALLOWED, "Invalid Method Error"),
        };

        HttpResponse::build(status_code)
            .json(json!({
                "error": error_type,
                "message": self.to_string(),
                "status_code": status_code.as_u16()
            }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidMethodError(_) => StatusCode::METHOD_NOT_ALLOWED,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

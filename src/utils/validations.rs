use crate::utils::errors::AppError;
use::regex::Regex;
use lazy_static::lazy_static;

// email validation
lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
}

pub fn is_valid_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

// public routes
pub fn is_public_route(path: &str) -> bool {
    let public_routes = [
        "/api/auth/login",
    ];
    public_routes.iter().any(|route| path.starts_with(route))
}

pub fn validate_required_fields(validations: &[(&str, bool)], error_prefix: &str) -> Result<(), AppError> {
    for (field_name, is_empty) in validations {
        if *is_empty {
            return Err(AppError::BadRequest(
                format!("{}: {} cannot be empty", error_prefix, field_name)
            ));
        }
    }
    Ok(())
}

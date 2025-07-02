use actix_web::web;
use crate::routes::{users, auth};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(users::configure_routes)
            .configure(auth::configure_routes)
    );
}

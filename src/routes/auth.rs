use actix_web::web;
use crate::controllers::auth;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/login")
                    .route(web::post().to(auth::login))
            )
    );
}

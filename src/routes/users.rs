use actix_web::web;
use crate::controllers::users;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(
                web::resource("")
                    .route(web::post().to(users::create_user))
                    .route(web::get().to(users::find_all_users))
            )
            .service(
                web::resource("/{id}")
                    .route(web::put().to(users::update_user))
                    .route(web::get().to(users::find_user_by_id))
                    .route(web::delete().to(users::delete_user_by_id))
            )
    );
}

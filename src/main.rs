use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use env_logger::{Builder, Env};
use log::info;
use rust_api_architecture_model::adapters::password_hasher::Argon2PasswordEncryptor;
use rust_api_architecture_model::adapters::token_hasher::JwtTokenGenerator;
use rust_api_architecture_model::config::{config_env::Config, database::init_database};
use rust_api_architecture_model::repositories::user_repository::PgUserRepository;
use rust_api_architecture_model::routes::config::base_routes::configure_routes;
use rust_api_architecture_model::services::user_service::UserService;
use rust_api_architecture_model::services::auth_service::AuthService;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env::default()
        .filter_or("RUST_LOG", "info,actix_web=info");

    Builder::from_env(env)
        .format_timestamp_millis()
        .format_module_path(true)
        .init();

    dotenv::dotenv().ok();

    let config = Config::from_env();

    let pool = init_database(&config.database_url).await;
    info!("Database connection established");

    //Create adapters
    let password_encryptor = Box::new(Argon2PasswordEncryptor::new());

    //Create repositories
    let user_repository = web::Data::new(PgUserRepository::new(pool.clone()));
    info!("Repositories Created");

    //Create services
    let user_service = web::Data::new(UserService::new(
          user_repository.clone(),
          password_encryptor.clone(),
    ));

    let auth_service = web::Data::new(AuthService::new(
        user_repository.clone(),
        web::Data::new(config.clone()),
        Box::new(Argon2PasswordEncryptor::new()),
        Box::new(JwtTokenGenerator::new()),
    ));

    //Start the server
    let server_addr = config.server_addr.clone();
    info!("Server will be started at: http://{}", server_addr);

    HttpServer::new(move || {
        //Define the Cors
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(user_repository.clone())
            .app_data(user_service.clone())
            .app_data(auth_service.clone())
            .configure(configure_routes)
    })
    .bind(server_addr)?
    .run()
    .await
}

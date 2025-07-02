use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn init_database(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Falied to create pool database")
}

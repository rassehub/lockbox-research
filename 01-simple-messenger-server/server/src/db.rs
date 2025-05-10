// db.rs
use sqlx::{postgres::PgPoolOptions, PgPool};
use dotenv::dotenv;
use std::env;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://secureuser:securepass@localhost/securechat".into());

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
}

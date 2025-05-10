use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPool;
mod db;
mod models;
mod chat_server;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize DB
    let pool = db::create_pool().await.expect("Failed to connect to DB");
    
    // Run migrations
    sqlx::migrate!().run(&pool).await.expect("Failed to run migrations");

    // Create chat server
    let server = web::Data::new(chat_server::ChatServer::new(pool));

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .route("/ws/{username}", web::get().to(handlers::chat_ws))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
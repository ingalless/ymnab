mod views;
mod handlers;
mod db;
mod app;
mod helpers;

use std::env;

use dotenvy::dotenv;
use poem::{ 
    listener::TcpListener,
    Result, Server,
};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().unwrap();
    let database_url = env::var("DATABASE_URL").expect("No database url set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url.to_string())
        .await
        .expect("Could not connect to DB");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app::app(pool))
        .await
}

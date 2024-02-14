mod views;
mod handlers;
mod db;
mod app;

use poem::{ 
    listener::TcpListener,
    Result, Server,
};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let pool = SqlitePoolOptions::new().max_connections(5).connect("sqlite://database.db").await.expect("Could not connect to DB");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app::app(pool))
        .await
}

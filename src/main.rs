use db::User;
use poem::{get, handler, http::StatusCode, listener::TcpListener, middleware::AddData, post, web::{Data, Form, Html, Json}, EndpointExt, IntoResponse, Response, Result, Route, Server};
use serde::Deserialize;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
mod views;
mod db;

#[handler]
fn login_page() -> Html<String> {
    Html(views::login().into())
}

#[derive(Deserialize)]
struct Login {
    email: String,
    password: String
}

#[handler]
async fn login(pool: Data<&Pool<MySql>>, params: Form<Login>) -> Response {
    // TODO: Redirect to dashboard on success, flash error on unsuccessful 
    let user = db::auth_user(&pool, params.email.to_owned(), params.password.to_owned()).await;
    match user {
        Some(_) => Response::default()
            .set_status(StatusCode::OK)
            .with_header("HX-Location", "/")
            .into_response(),
        None => Html(views::login_error().into_string()).into_response()
    }
}

#[handler]
async fn sign_up(pool: Data<&Pool<MySql>>, req: Json<User>) {
    // let user = User::from_form(req.name.to_owned(), req.email.to_owned(), req.password.to_string());
    // db::create_user(&pool, user).await;
}

#[handler]
fn home() -> Html<String> {
    Html(views::home().into())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let pool = MySqlPoolOptions::new().max_connections(5).connect("mysql://root:password@localhost/ymnab").await.expect("Could not connect to DB");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    match db::get_user(&pool, "jonny@example.com".to_string()).await {
        None => db::create_user(&pool, User::from_form("jonny".to_string(), "jonny@example.com".to_string(), "password".to_string()).expect("Failed to seed user.")).await,
        _ => {}
    }

    let app = Route::new()
        .at("/", get(home))
        .at("/login", get(login_page).post(login))
        .at("/sign_up", post(sign_up))
        .with(AddData::new(pool));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

use db::User;
use maud::html;
use poem::{get, handler, 
    http::{header, StatusCode},
    listener::TcpListener, middleware::AddData,
    web::{Data, Form, Html},
    EndpointExt, IntoResponse, Response, Result, Route, Server,
    session::{CookieConfig, CookieSession, Session}
};
use serde::Deserialize;
use sqlx::{sqlite::SqlitePoolOptions, Sqlite, Pool};
mod views;
mod db;

#[handler]
fn login_page(session: &Session) -> impl IntoResponse {
    if session.get::<String>("user").is_some() {
        Response::builder()
                .status(StatusCode::FOUND)
                .header(header::LOCATION, "/")
                .finish()
    } else {
        Html(views::login().into_string()).into_response()
    }
}

#[handler]
fn sign_up_page(session: &Session) -> impl IntoResponse {
    if session.get::<String>("user").is_some() {
        Response::builder()
                .status(StatusCode::FOUND)
                .header(header::LOCATION, "/")
                .finish()
    } else {
        Html(views::signup().into_string()).into_response()
    }
}

#[derive(Deserialize)]
struct Login {
    email: String,
    password: String
}

#[handler]
async fn login(pool: Data<&Pool<Sqlite>>, session: &Session, params: Form<Login>) -> Response {
    let user = db::auth_user(&pool, params.email.to_owned(), params.password.to_owned()).await;
    match user {
        Some(u) => {
            session.set("user", u.email);
            Response::default()
                .set_status(StatusCode::OK)
                .with_header("HX-Location", "/")
                .into_response()
        },
        None => Html(views::error_message("User not found").into_string()).into_response()
    }
}

#[derive(Deserialize)]
struct Signup {
    name: String,
    email: String,
    password: String,
}
#[handler]
async fn sign_up(pool: Data<&Pool<Sqlite>>, params: Form<Signup>) -> impl IntoResponse {
    match User::from_form(params.name.to_owned(), params.email.to_owned(), params.password.to_string()) {
        Ok(u) => {
            match db::create_user(&pool, u).await {
                Ok(_) => Response::default()
                    .set_status(StatusCode::OK)
                    .with_header("HX-Location", "/")
                    .into_response(),
                _ => Html(html! { p class="text-red-600 font-semibold" { "Failed to create user." } }).into_response()
            }
        },
        Err(_) => Html(html! { p class="text-red-600 font-semibold" { "Failed to create user." } }).into_response()
    }
}

#[handler]
fn home(session: &Session) -> impl IntoResponse {
    if session.get::<String>("user").is_none() {
        Response::builder()
                    .status(StatusCode::FOUND)
                    .header(header::LOCATION, "/login")
                    .finish()
    } else {
        Html(views::home().into_string()).into_response()
    }
}

#[handler]
fn logout(session: &Session) -> impl IntoResponse {
    session.remove("user");
    Response::builder()
                .status(StatusCode::FOUND)
                .header(header::LOCATION, "/login")
                .finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let pool = SqlitePoolOptions::new().max_connections(5).connect("sqlite://database.db").await.expect("Could not connect to DB");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    let app = Route::new()
        .at("/", get(home))
        .at("/login", get(login_page).post(login))
        .at("/signup", get(sign_up_page).post(sign_up))
        .at("/logout", get(logout))
        .with(AddData::new(pool))
        .with(CookieSession::new(CookieConfig::default().secure(false)));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

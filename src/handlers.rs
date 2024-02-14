use maud::html;
use poem::{handler, http::{header, StatusCode}, session::Session, web::{Data, Form, Html}, IntoResponse, Response};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};

use crate::{db::User, views};
use crate::db;

#[handler]
pub fn login_page(session: &Session) -> impl IntoResponse {
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
pub fn sign_up_page(session: &Session) -> impl IntoResponse {
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
pub async fn login(pool: Data<&Pool<Sqlite>>, session: &Session, params: Form<Login>) -> Response {
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
pub async fn sign_up(pool: Data<&Pool<Sqlite>>, params: Form<Signup>) -> impl IntoResponse {
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
pub fn home(session: &Session) -> impl IntoResponse {
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
pub fn logout(session: &Session) -> impl IntoResponse {
    session.remove("user");
    Response::builder()
                .status(StatusCode::FOUND)
                .header(header::LOCATION, "/login")
                .finish()
}

use maud::html;
use poem::{handler, http::{header, StatusCode}, session::Session, web::{Data, Form, Html}, IntoResponse, Response};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};

use crate::{db::User, helpers::get_total_as_formatted_string, views::{self, simple_error}};
use crate::db;

fn needs_login(session: &Session) -> bool {
    match session.get::<String>("user") {
        Some(_) => false,
        None => true
    }
}

fn redirect(location: &str) -> Response {
    Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, location)
        .finish()
}

fn redirect_to_login() -> Response {
    Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, "/login")
        .finish()
}

fn redirect_to_home() -> Response {
    Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, "/")
        .finish()
}

#[derive(Deserialize)]
struct CreateAccountBody {
    name: String,
    starting_balance: String,
}

#[handler]
pub async fn create_account(pool: Data<&Pool<Sqlite>>, session: &Session, data: Form<CreateAccountBody>) -> impl IntoResponse {
    if needs_login(session) {
        return StatusCode::UNAUTHORIZED.into();
    }

    let user = db::get_user(&pool, session.get("user").unwrap()).await.unwrap();

    let starting_balance: i32 = data.starting_balance.trim().replace(",", "").replace(".", "").parse().unwrap();

    match db::create_account(&pool, user.id.unwrap(), &data.name, starting_balance).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(message) => {
            println!("{}", message);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[handler]
pub fn login_page(session: &Session) -> impl IntoResponse {
    if !needs_login(session) {
        redirect_to_home()
    } else {
        Html(views::login().into_string()).into_response()
    }
}

#[handler]
pub fn sign_up_page(session: &Session) -> impl IntoResponse {
    if !needs_login(session) {
        redirect_to_home()
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
                .with_header("HX-Redirect", "/")
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
pub async fn home(pool: Data<&Pool<Sqlite>>, session: &Session) -> impl IntoResponse {
    if needs_login(session) {
        return redirect_to_login();
    }

    let user = db::get_user(&pool, session.get("user").unwrap()).await;
    if user.is_none() {
        return Html(simple_error("Could not get user.")).into_response();
    }
    let accounts = db::get_accounts_for_user(&pool, user.unwrap().id.unwrap()).await;
    if accounts.is_none() {
        return Html(simple_error("Could not get accounts.")).into_response();
    }

    let budget_total = accounts.as_ref().unwrap().iter().fold(0, |acc, x| {
        acc + x.total
    });

    Html(views::home(accounts.unwrap(), get_total_as_formatted_string(budget_total)).into_string()).into_response()
}

#[handler]
pub fn logout(session: &Session) -> impl IntoResponse {
    session.remove("user");
    Response::builder()
                .status(StatusCode::FOUND)
                .header(header::LOCATION, "/login")
                .finish()
}

use db::User;
use poem::{get, handler, http::StatusCode, listener::TcpListener, middleware::AddData, post, web::{Html, Json, Data}, EndpointExt, IntoResponse, Response, Result, Route, Server};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
mod views;
mod db;

#[handler]
fn login_page() -> Html<String> {
    Html(views::login().into())
}

#[handler]
fn login(pool: Data<&Pool<MySql>>, req: Json<User>) -> Response {
    // TODO: Redirect to dashboard on success, flash error on unsuccessful 
    let user = db::auth_user(&pool, req.email.to_owned(), req.password.as_ref().unwrap().to_string());
    Response::default()
        .set_status(StatusCode::SEE_OTHER)
        .with_header("Location", "/")
        .into_response()
}

#[handler]
async fn sign_up(pool: Data<&Pool<MySql>>, req: Json<User>) {
    let user = User::from_form(req.name.to_owned(), req.email.to_owned(), req.password.as_ref().unwrap().to_string());
    db::create_user(&pool, user).await;
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

    let app = Route::new()
        .at("/", get(home))
        .at("/login", get(login_page).post(login))
        .at("/sign_up", post(sign_up))
        .with(AddData::new(pool));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

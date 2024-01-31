use poem::{get, handler, http::StatusCode, listener::TcpListener, web::Html, IntoResponse, Response, Result, Route, Server};
mod views;

#[handler]
fn login_page() -> Html<String> {
    Html(views::login().into())
}

#[handler]
fn login() -> Response {
    Response::default()
        .set_status(StatusCode::SEE_OTHER)
        .with_header("Location", "/")
        .into_response()
}

#[handler]
fn home() -> Html<String> {
    Html(views::home().into())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/", get(home))
        .at("/login", get(login_page).post(login));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

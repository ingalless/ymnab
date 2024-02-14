use poem::{get, middleware::AddData, session::{CookieConfig, CookieSession}, EndpointExt, IntoEndpoint, Route
};
use sqlx::{Pool, Sqlite};
use crate::handlers::{home, login_page, login, sign_up_page, sign_up, logout};

pub fn app(pool: Pool<Sqlite>) -> impl IntoEndpoint {
    let app = Route::new()
        .at("/", get(home))
        .at("/login", get(login_page).post(login))
        .at("/signup", get(sign_up_page).post(sign_up))
        .at("/logout", get(logout))
        .with(AddData::new(pool))
        .with(CookieSession::new(CookieConfig::default().secure(false)));

    return app;
}

#[cfg(test)] 
mod tests {
    use poem::test::TestClient;
    use serde::Serialize;

    use crate::db::User;

    use super::*;

    #[derive(Serialize)]
    struct Login<'a> {
        email: &'a str,
        password: &'a str,
    }

    #[sqlx::test]
    async fn test_login(pool: Pool<Sqlite>) -> sqlx::Result<()> {
        // Setup
        let password = User::hash_password("password".to_string()).expect("Could not hash password");
        sqlx::query("INSERT INTO users (name, email, password, active) VALUES ('test', 'test@example.com', ?, 1)")
            .bind(password)
            .execute(&pool)
            .await?;

        // Act
        let cli = TestClient::new(app(pool));
        let resp = cli
            .post("/login")
            .form(&Login {
                email: "test@example.com",
                password: "password"
            })
            .send()
            .await;
        
        // Assert
        resp.assert_status_is_ok();
        resp.assert_header("HX-Location", "/");

        Ok(())
    }
}


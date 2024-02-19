use poem::{get, middleware::AddData, session::{CookieConfig, CookieSession}, EndpointExt, IntoEndpoint, Route
};
use sqlx::{Pool, Sqlite};
use crate::handlers::{home, login_page, login, sign_up_page, sign_up, logout};

pub fn app(pool: Pool<Sqlite>) -> impl IntoEndpoint {
    Route::new()
        .at("/", get(home))
        .at("/login", get(login_page).post(login))
        .at("/signup", get(sign_up_page).post(sign_up))
        .at("/logout", get(logout))
        .with(AddData::new(pool))
        .with(CookieSession::new(CookieConfig::default().secure(false)))
}

#[cfg(test)] 
mod tests {
    use poem::test::TestClient;
    use serde::{Serialize, Deserialize};

    use crate::db::{User, _get_user};

    use super::*;

    #[derive(Serialize)]
    struct Login<'a> {
        email: &'a str,
        password: &'a str,
    }

    #[derive(Serialize, Deserialize)]
    struct Signup<'a> {
        name: &'a str,
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
        resp.assert_header("HX-Redirect", "/");

        Ok(())
    }

    #[sqlx::test]
    async fn test_invalid_login(pool: Pool<Sqlite>) -> sqlx::Result<()> {
        // Setup
        let password = User::hash_password("password".to_string()).expect("Could not hash password");
        sqlx::query("INSERT INTO users (name, email, password, active) VALUES ('test', 'test@example.com', ?, 1)")
            .bind(password)
            .execute(&pool)
            .await?;

        // Act
        let cli = TestClient::new(app(pool));
        let invalid_password_response = cli
            .post("/login")
            .form(&Login {
                email: "test@example.com",
                password: "password123"
            })
            .send()
            .await;
        let password_body = invalid_password_response.0.into_body();

        let invalid_username_response = cli
            .post("/login")
            .form(&Login {
                email: "test@notexample.com",
                password: "password"
            })
            .send()
            .await;
        let username_body = invalid_username_response.0.into_body();

        let invalid_both_response = cli
            .post("/login")
            .form(&Login {
                email: "not@notexample.com",
                password: "notpassword"
            })
            .send()
            .await;
        let both_body = invalid_both_response.0.into_body();

        assert_eq!(username_body.into_string().await.unwrap().contains("User not found"), true, "Incorrect response when username is invalid");
        assert_eq!(password_body.into_string().await.unwrap().contains("User not found"), true, "Incorrect response when password is invalid");
        assert_eq!(both_body.into_string().await.unwrap().contains("User not found"), true, "Incorrect response when both username and password are invalid");

        Ok(())
    }

    #[sqlx::test]
    async fn test_signup(pool: Pool<Sqlite>) -> sqlx::Result<()> {
        let cli = TestClient::new(app(pool.clone()));
        let response = cli
            .post("/signup")
            .form(&Signup {
                name: "Test",
                email: "test@example.com",
                password: "password123"
            })
            .send()
            .await;

        response.assert_status_is_ok();

        let user = _get_user(&pool, "test@example.com".to_string()).await.unwrap();

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test");
        assert_eq!(user.active, true);

        Ok(())
    }
}


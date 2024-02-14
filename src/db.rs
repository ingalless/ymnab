use bcrypt;
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Pool};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub active: bool,

}

#[derive(Debug)]
pub struct HashError;

impl User {
    pub fn from_form(name: String, email: String, password: String) -> Result<Self, HashError> {
        match Self::hash_password(password) {
            Ok(v) => Ok(Self {
                name,
                email,
                password: v,
                active: true
            }),
            _ => Err(HashError)
        }
    }

    fn from_db(name: String, email: String, password: String, active: bool) -> Self {
        Self {
            name,
            email,
            active,
            password,
        }
    }

    pub fn hash_password(password: String) -> Result<String, HashError> {
        match bcrypt::hash(&password.as_bytes(), 10) {
            Ok(v) => Ok(v),
            _ => Err(HashError)
        }
    }
}

pub async fn auth_user(conn: &Pool<Sqlite>, email: String, password: String) -> Option<User> {
    let result: Result<(String, String, String, bool), sqlx::Error> = sqlx::query_as("SELECT name, email, password, active FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(conn)
        .await;

    let user = match result {
        Ok(row) => Some(User::from_db(row.0, row.1, row.2, row.3)),
        Err(_) => None
    };

    match user {
        Some(u) => {
            match bcrypt::verify(password, &u.password) {
                Ok(true) => Some(u),
                _ => {
                    println!("Pass does not match");
                    None
                }
            }
        },
        None => None
    }
}

pub async fn _get_user(conn: &Pool<Sqlite>, email: String) -> Option<User> {
    let result: Result<(String, String, String, bool), sqlx::Error> = sqlx::query_as("SELECT name, email, password, active FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(conn)
        .await;

    match result {
        Ok(row) => Some(User::from_db(row.0, row.1, row.2, row.3)),
        Err(_) => None
    }
}

#[derive(Debug)]
pub struct CreateError;

pub async fn create_user(conn: &Pool<Sqlite>, user: User) -> Result<bool, CreateError> {
    let result = sqlx::query("INSERT INTO users (name, email, password, active) VALUES (?, ?, ?, ?)")
        .bind(user.name)
        .bind(user.email)
        .bind(user.password)
        .bind(user.active)
        .execute(conn)
        .await;

    match result {
        Ok(_) => Ok(true),
        Err(_) => Err(CreateError)
    }
}


#[sqlx::test]
async fn test_get_user(pool: Pool<Sqlite>) -> sqlx::Result<()> {
    // Setup
    sqlx::query("INSERT INTO users (name, email, password, active) VALUES ('test', 'test@example.com', '', 1)").execute(&pool).await?;

    // Act
    let user = get_user(&pool, "test@example.com".to_string()).await.expect("User is not found.");

    // Assert
    assert_eq!(user.name, "test");

    Ok(())
}

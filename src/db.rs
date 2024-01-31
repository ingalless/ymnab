 use argon2::{
     password_hash::{
         rand_core::OsRng,
         PasswordHash, PasswordHasher, SaltString
     },
     Argon2
 };
use serde::Deserialize;
use sqlx::{MySql, Pool};

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub active: bool,

}

impl User {
    pub fn from_form(name: String, email: String, password: String) -> Self {
        Self {
            name,
            email,
            password: Some(Self::hash_password(password).unwrap()),
            active: true
        }
    }

    fn from_db(name: String, email: String, active: bool) -> Self {
        Self {
            name,
            email,
            active,
            password: None,
        }
    }

    fn hash_password(password: String) -> Option<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        match &argon.hash_password(password.as_bytes(), &salt) {
            Ok(v) => Some(v.to_string()),
            &Err(_) => None
        }
    }
}

pub async fn auth_user(conn: &Pool<MySql>, email: String, password: String) -> Option<User> {
    let parsed_hash = PasswordHash::new(&password).unwrap();
    let result: (String, String, bool) = sqlx::query_as("SELECT name, email, active FROM users WHERE email = ? AND password = ?")
        .bind(email)
        .bind(parsed_hash.to_string())
        .fetch_one(conn)
        .await
        .unwrap();

    Some(User::from_db(result.0, result.1, result.2))
}

pub async fn create_user(conn: &Pool<MySql>, user: User) {
    sqlx::query("INSERT INTO users (name, email, password, active) VALUES (?, ?, ?, ?)")
        .bind(user.name)
        .bind(user.email)
        .bind(user.password)
        .bind(user.active)
        .execute(conn)
        .await
        .unwrap();
}

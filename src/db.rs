use bcrypt;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<i32>,
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
                id: None,
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
            id: None,
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

pub async fn get_user(conn: &Pool<Sqlite>, email: String) -> Option<User> {
    let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(conn)
        .await;

    match result {
        Ok(row) => Some(row),
        Err(_) => None
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: i32,
    pub name: String
}

pub async fn get_accounts_for_user(conn: &Pool<Sqlite>, id: i32) -> Option<Vec<Account>> {
    let results = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = ?")
        .bind(id)
        .fetch_all(conn)
        .await;

    match results {
        Ok(r) => Some(r),
        _ => None
    }
}

pub async fn create_account(conn: &Pool<Sqlite>, user_id: i32, name: &str, starting_balance: i32) -> Result<(), &'static str> {
    let insert_result = sqlx::query("INSERT INTO accounts (user_id, name) values (?, ?)")
        .bind(user_id)
        .bind(name)
        .execute(conn)
        .await;

    if insert_result.is_err() {
        return Err("failed to create account");
    }

    let starting_balance_result = sqlx::query("INSERT INTO transactions (account_id, date, memo, inflow, cleared) values (?, datetime(), ?, ?, ?)")
        .bind(insert_result.as_ref().unwrap().last_insert_rowid())
        .bind("Starting balance")
        .bind(starting_balance)
        .bind(true)
        .execute(conn)
        .await;

    return match starting_balance_result {
        Ok(_) => Ok(()),
        _=> Err("failed to create starting balance"),
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

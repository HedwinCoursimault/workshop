use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    #[serde(skip)]
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct UserInsertable {
    pub name: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub async fn get_user(conn: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    query_as!(User, "SELECT * FROM user_ WHERE id = $1", id)
        .fetch_optional(conn)
        .await
}

pub async fn login(
    conn: &PgPool,
    name: &str,
    password: &str,
) -> Result<User, String> {
    let result = query_as!(
        User,
        "SELECT * FROM user_ WHERE name = $1 AND password = crypt($2, password)",
        name,
        password
    )
    .fetch_one(conn)
    .await;

    match result {
        Err(e) => {
            if let sqlx::Error::RowNotFound = e { return Err("no account found".to_string()) };
        }
        _ => return result.map_err(|e| e.to_string()),
    };

    Err("no account found".to_string())
}

pub async fn insert_user(conn: &PgPool, new: &UserInsertable) -> Result<User, sqlx::Error> {
    query_as!(
        User,
        "INSERT INTO user_ (name, password) 
        VALUES ($1, crypt($2, gen_salt('bf'))) 
        RETURNING *",
        new.name,
        new.password
    )
    .fetch_one(conn)
    .await
}
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInsertable {
    pub name: String,
}

pub async fn get_user(conn: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    query_as!(User, "SELECT * FROM user_ WHERE id = $1", id)
        .fetch_optional(conn)
        .await
}

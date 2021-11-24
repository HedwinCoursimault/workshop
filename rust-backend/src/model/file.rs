use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub uploader: Uuid,
    pub times_downloaded: i32,
}

#[derive(Debug, Deserialize)]
pub struct FileInsertable<'a> {
    pub name: &'a str,
    pub uploader: Uuid,
}

pub async fn get_file(conn: &PgPool, id: &Uuid) -> Result<Option<File>, sqlx::Error> {
    query_as!(File, "SELECT * FROM file_ WHERE id = $1", id)
        .fetch_optional(conn)
        .await
}

pub async fn get_file_by_name(conn: &PgPool, name: &str) -> Result<Option<File>, sqlx::Error> {
    query_as!(File, "SELECT * FROM file_ WHERE name = $1", name)
        .fetch_optional(conn)
        .await
}

pub async fn insert_file(conn: &PgPool, new: &FileInsertable<'_>) -> Result<File, sqlx::Error> {
    query_as!(
        File,
        "INSERT INTO file_ (name, uploader)
        VALUES ($1, $2)
        RETURNING *",
        new.name,
        new.uploader
    )
    .fetch_one(conn)
    .await
}

pub async fn bump_download_counter(conn: &PgPool, file_id: &Uuid) -> Result<i32, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE file_ set times_downloaded = times_downloaded + 1 RETURNING times_downloaded"
    )
    .fetch_one(conn)
    .await?;

    Ok(result.times_downloaded)
}

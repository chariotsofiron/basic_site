use sqlx::SqlitePool;

use crate::app_state::Timestamp;

pub struct UserRecord {
    pub id: u32,
    pub username: String,
    pub password_hash: String,
    pub created_at: i64,
}

pub async fn get_user_by_id(db: &SqlitePool, id: u32) -> Result<Option<UserRecord>, sqlx::Error> {
    sqlx::query_as!(
        UserRecord,
        r#"
        SELECT
            id as "id: u32",
            username,
            password_hash,
            created_at
        FROM 'user'
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(db)
    .await
}

pub async fn get_user_by_username(
    db: &SqlitePool,
    username: &str,
) -> Result<UserRecord, sqlx::Error> {
    sqlx::query_as!(
        UserRecord,
        r#"
        SELECT
            id as "id: u32",
            username,
            password_hash,
            created_at
        FROM 'user'
        WHERE username = ?
        "#,
        username
    )
    .fetch_one(db)
    .await
}

pub async fn insert(
    db: &SqlitePool,
    username: &str,
    password_hash: &str,
    created_at: Timestamp,
) -> Result<u32, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO user (username, password_hash, created_at)
        VALUES (?, ?, ?)
        "#,
        username,
        password_hash,
        created_at
    )
    .execute(db)
    .await
    .map(|row| u32::try_from(row.last_insert_rowid()).unwrap())
}

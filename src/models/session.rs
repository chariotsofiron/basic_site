use sqlx::SqlitePool;
use tracing::info;

use crate::app_state::{Timestamp, UserId};

#[derive(sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: UserId,
    // pub user_agent: String,
    // pub created_at: Timestamp,
    // pub expires_at: Timestamp,
}

pub async fn get_session_by_id(db: &SqlitePool, id: &str) -> Result<Option<Session>, sqlx::Error> {
    sqlx::query_as::<_, Session>("SELECT * FROM 'session' WHERE id = ?")
        .bind(id)
        .fetch_optional(db)
        .await
}

pub async fn insert(
    db: &SqlitePool,
    session_id: &str,
    user_id: UserId,
) -> Result<i64, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO session (id, user_id) VALUES (?, ?)",
        session_id,
        user_id
    )
    .execute(db)
    .await
    .map(|row| row.last_insert_rowid())
}

/// Don't need to check if correct user because guessing is unlikely.
pub async fn delete(db: &SqlitePool, session_id: &str) -> Result<u64, sqlx::Error> {
    info!("deleting session id {}", session_id);
    sqlx::query!("DELETE FROM session WHERE id = ?", session_id)
        .execute(db)
        .await
        .map(|row| row.rows_affected())
}

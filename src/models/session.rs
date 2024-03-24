use crate::app_state::{Timestamp, UserId};
use sqlx::SqlitePool;
use tracing::info;

#[derive(sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: UserId,
    pub ip_address: String,
    pub user_agent: String,
    pub created_at: Timestamp,
    pub expires_at: Timestamp,
}

impl Session {
    pub async fn get_by_id(db: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM 'session' WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
    }

    /// Don't need to check if correct user because guessing is unlikely.
    pub async fn delete_by_id(db: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        info!("deleting session id {id}");
        sqlx::query!("DELETE FROM session WHERE id = ?", id)
            .execute(db)
            .await
            .map(|row| row.rows_affected())
    }

    pub async fn get_all_for_user(
        db: &SqlitePool,
        user_id: UserId,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM 'session' WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(db)
            .await
    }

    pub async fn insert(&self, db: &SqlitePool) -> Result<i64, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO session (id, user_id, ip_address, user_agent, created_at, expires_at) VALUES (?, ?, ?, ?, ?, ?)",
            self.id,
            self.user_id,
            self.ip_address,
            self.user_agent,
            self.created_at,
            self.expires_at,
        )
            .execute(db)
            .await
            .map(|row| row.last_insert_rowid())
    }
}

use sqlx::SqlitePool;

use crate::app_state::{Timestamp, UserId};

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password_hash: String,
    pub created_at: Timestamp,
}

impl User {
    pub async fn get_by_id(db: &SqlitePool, id: UserId) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM 'user' WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
    }

    pub async fn get_by_username(db: &SqlitePool, username: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM 'user' WHERE username = ?")
            .bind(username)
            .fetch_one(db)
            .await
    }

    pub async fn insert(&self, db: &SqlitePool) -> Result<UserId, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO user (username, password_hash, created_at)
            VALUES (?, ?, ?)
            "#,
            self.username,
            self.password_hash,
            self.created_at
        )
        .execute(db)
        .await
        .map(|row| UserId::try_from(row.last_insert_rowid()).unwrap())
    }
}

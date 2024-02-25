use sqlx::SqlitePool;

pub struct SessionRecord {
    pub id: String,
    pub user_id: u32,
}

pub async fn get_session_by_id(db: &SqlitePool, id: &str) -> Result<SessionRecord, sqlx::Error> {
    sqlx::query_as!(
        SessionRecord,
        r#"
        SELECT
            id,
            user_id as "user_id: u32"
        FROM 'session'
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(db)
    .await
}

pub async fn insert(db: &SqlitePool, session_id: &str, user_id: u32) -> Result<i64, sqlx::Error> {
    sqlx::query!(
        "INSERT INTO session (id, user_id) VALUES (?, ?)",
        session_id,
        user_id
    )
    .execute(db)
    .await
    .map(|row| row.last_insert_rowid())
}

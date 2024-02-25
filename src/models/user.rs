use sqlx::SqlitePool;

pub struct UserRecord {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub created_at: i64,
}

pub async fn get_user_by_id(db: &SqlitePool, id: u32) -> Result<UserRecord, sqlx::Error> {
    sqlx::query_as!(
        UserRecord,
        r#"
        SELECT
            id as "id: u32",
            username,
            password,
            created_at
        FROM 'user'
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(db)
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
            password,
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
    password: &str,
    created_at: i64,
) -> Result<u32, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO user (username, password, created_at)
        VALUES (?, ?, ?)
        "#,
        username,
        password,
        created_at
    )
    .execute(db)
    .await
    .map(|row| row.last_insert_rowid() as u32)
}

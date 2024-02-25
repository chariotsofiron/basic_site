use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[derive(Clone)]
pub struct AppState {
    database: Pool<Sqlite>,
}

/// Connects to the database using the `DATABASE_URL` environment variable.
async fn connect_db() -> Pool<Sqlite> {
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set");
    SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to database")
}

impl AppState {
    pub async fn build() -> Self {
        let database = connect_db().await;
        Self { database }
    }

    // /// Returns Ok(user_id) if the session is valid, otherwise Err(sqlx::Error).
    // pub fn authenticate(&self, session_id: &str) -> Result<i64, sqlx::Error> {
    //     // if let Some(cookie) = jar.get("session_id") {
    //     //     let session_id = cookie.value();
    //     //     match sqlx::query!("SELECT user_id FROM session WHERE id = ?", session_id)
    //     //         .fetch_one(&state.database)
    //     //         .await
    //     //     {
    //     //         Ok(row) => {
    //     //             let user_id = row.user_id;
    //     //             return Html(HomePage::make_page_logged_in("bob")).into_response();
    //     //         }
    //     //         Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    //     //     }

    //     // sqlx::query!("SELECT user_id FROM session WHERE id = ?", session_id)
    //     //     .fetch_one(&self.database)
    //     //     .map(|row| row.user_id)

    //     Ok(1)
    // }
}

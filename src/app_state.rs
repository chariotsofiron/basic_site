use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::models::{
    session_record::{self, get_session_by_id},
    user_record::{self, get_user_by_id, UserRecord},
};

#[derive(Clone)]
pub struct AppState {
    pub database: Pool<Sqlite>,
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

    pub async fn authenticate(&self, jar: CookieJar) -> Option<UserRecord> {
        let session_id = jar.get("session_id")?.value();
        let session = get_session_by_id(&self.database, session_id).await.ok()?;
        let user = get_user_by_id(&self.database, session.user_id).await.ok()?;

        Some(user)
    }

    /// Create a session id for the user.
    pub async fn make_auth_session(&self, user_id: u32) -> Cookie<'static> {
        // random 128-bit hex value
        let session_id = format!("{:#018x}", rand::random::<u128>());
        session_record::insert(&self.database, &session_id, user_id)
            .await
            .expect("failed to insert session id into database");

        let cookie = Cookie::build(("session_id", session_id))
            .path("/")
            .same_site(SameSite::Strict)
            .http_only(true)
            .max_age(time::Duration::WEEK)
            .build();
        cookie
    }

    /// Creates a new session id
    pub async fn login(&self, username: &str, password: &str) -> Option<Cookie<'static>> {
        match user_record::get_user_by_username(&self.database, username).await {
            Ok(user) if user.password == password => {
                let cookie = self.make_auth_session(user.id).await;
                Some(cookie)
            }
            Ok(_) | Err(_) => None,
        }
    }
}

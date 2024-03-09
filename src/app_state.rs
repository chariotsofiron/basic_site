use argon2::PasswordVerifier;
use argon2::{Argon2, PasswordHash};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::models::{
    session_record::{self, get_session_by_id},
    user_record::{self, get_user_by_id, UserRecord},
};

pub type Timestamp = i64;

#[derive(Clone)]
pub struct AppState {
    pub database: Pool<Sqlite>,
}

/// Returns the current time in microseconds.
pub fn timestamp_micros() -> Timestamp {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros() as Timestamp
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

    /// Generates a random 128-bit hex string.
    fn generate_session_id() -> String {
        format!("{:#018x}", rand::random::<u128>())
    }

    fn build_session_cookie(session_id: &str) -> Cookie<'static> {
        Cookie::build(("session_id", session_id))
            .path("/")
            .same_site(SameSite::Strict)
            .http_only(true)
            .max_age(time::Duration::WEEK)
            .build()
            .into_owned()
    }

    /// Create a session id for the user and return a cookie for it.
    pub async fn make_auth_session(&self, user_id: u32) -> Cookie<'static> {
        let session_id = Self::generate_session_id();

        session_record::insert(&self.database, &session_id, user_id)
            .await
            .unwrap();

        Self::build_session_cookie(&session_id)
    }

    /// Authenticate user and create a new session id
    pub async fn login(&self, username: &str, password: &str) -> Option<Cookie<'static>> {
        match user_record::get_user_by_username(&self.database, username).await {
            Ok(user) => {
                let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
                if Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_err()
                {
                    return None;
                }
                let cookie = self.make_auth_session(user.id).await;
                Some(cookie)
            }
            Err(_) => None,
        }
    }

    pub async fn logout(&self, session_id: &str) {
        session_record::delete(&self.database, session_id)
            .await
            .expect("failed to delete session id from database");
    }
}

use argon2::PasswordVerifier;
use argon2::{Argon2, PasswordHash};
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_extra::extract::{cookie::Cookie, cookie::SameSite, CookieJar};
use sqlx::SqlitePool;

use crate::app_state::UserId;
use crate::models::session::{self, get_session_by_id};
use crate::models::user::{self, get_user_by_id, User};

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
pub async fn make_auth_session(db: &SqlitePool, user_id: UserId) -> Cookie<'static> {
    let session_id = generate_session_id();

    session::insert(db, &session_id, user_id).await.unwrap();

    build_session_cookie(&session_id)
}

/// Authenticate user and create a new session id
pub async fn login(db: &SqlitePool, username: &str, password: &str) -> Option<Cookie<'static>> {
    match user::get_user_by_username(db, username).await {
        Ok(user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
            if Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_err()
            {
                return None;
            }
            let cookie = make_auth_session(db, user.id).await;
            Some(cookie)
        }
        Err(_) => None,
    }
}

pub async fn logout(db: &SqlitePool, session_id: &str) {
    session::delete(db, session_id)
        .await
        .expect("failed to delete session id from database");
}

pub struct UserExtractor(pub Option<User>);

#[async_trait]
impl<S> FromRequestParts<S> for UserExtractor
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();
        let pool: &SqlitePool = parts.extensions.get().unwrap();

        let Some(session_id) = jar.get("session_id").map(Cookie::value) else {
            return Ok(Self(None));
        };
        let session = match get_session_by_id(pool, session_id).await {
            Ok(Some(session)) => session,
            Ok(None) => return Ok(Self(None)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
        };
        let user = match get_user_by_id(pool, session.user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Ok(Self(None)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
        };

        Ok(Self(Some(user)))
    }
}

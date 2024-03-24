use argon2::PasswordVerifier;
use argon2::{Argon2, PasswordHash};
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_extra::extract::{cookie::Cookie, cookie::SameSite, CookieJar};
use axum_extra::headers::UserAgent;
use sqlx::SqlitePool;

use crate::app_state::{Timestamp, UserId};
use crate::models::session::Session;
use crate::models::user::User;

/// Generates a random 128-bit hex string.
fn generate_session_id() -> String {
    format!("{:#018x}", rand::random::<u128>())
}

fn build_session_cookie(session_id: &str) -> Cookie<'static> {
    Cookie::build(("session_id", session_id))
        .path("/")
        .same_site(SameSite::Strict)
        .secure(true)
        .http_only(true)
        .max_age(time::Duration::WEEK)
        .build()
        .into_owned()
}

/// Create a session for the user and return a cookie for it.
pub async fn create_session(
    db: &SqlitePool,
    user_id: UserId,
    time: Timestamp,
    ip_address: String,
    user_agent: UserAgent,
) -> Cookie<'static> {
    let id = generate_session_id();
    let session = Session {
        id: id.clone(),
        user_id,
        ip_address,
        user_agent: user_agent.to_string(),
        created_at: time,
        expires_at: 0, // todo
    };
    session.insert(db).await.unwrap();
    build_session_cookie(&id)
}

/// Authenticate user and create a new session id.
pub async fn login(
    db: &SqlitePool,
    username: &str,
    password: &str,
    time: Timestamp,
    ip_address: String,
    user_agent: UserAgent,
) -> Option<Cookie<'static>> {
    match User::get_by_username(db, username).await {
        Ok(user) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .ok()?;
            let cookie = create_session(db, user.id, time, ip_address, user_agent).await;
            Some(cookie)
        }
        Err(_) => None,
    }
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
        let session = match Session::get_by_id(pool, session_id).await {
            Ok(Some(session)) => session,
            Ok(None) => return Ok(Self(None)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
        };
        let user = match User::get_by_id(pool, session.user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return Ok(Self(None)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
        };

        Ok(Self(Some(user)))
    }
}

use std::net::SocketAddr;

use argon2::PasswordHasher;
use argon2::{password_hash::SaltString, Argon2};
use axum::extract::ConnectInfo;
use axum::Extension;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::CookieJar;
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use rand::rngs::OsRng;
use serde::Deserialize;
use sqlx::SqlitePool;
use tracing::warn;

use crate::app_state::timestamp_micros;
use crate::auth::{self, UserExtractor};
use crate::models::user::User;
use crate::templates::signup;
use crate::templates::signup_form;

pub async fn get(UserExtractor(user): UserExtractor) -> impl IntoResponse {
    match user {
        Some(_) => Redirect::to("/").into_response(),
        None => Html(signup::build()).into_response(),
    }
}

#[derive(Deserialize, Debug)]
pub struct Credentials {
    username: String,
    password: String,
}

pub async fn post(
    jar: CookieJar,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    db: Extension<SqlitePool>,
    Form(form): Form<Credentials>,
) -> impl IntoResponse {
    let timestamp = timestamp_micros();

    let username_message = validate_username(&form.username);
    let password_message = validate_password(&form.password);
    if !username_message.is_empty() || !password_message.is_empty() {
        return signup_form::build_with_error_message(
            &form.username,
            username_message,
            password_message,
        )
        .into_response();
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let Ok(password_hash) = argon2.hash_password(form.password.as_bytes(), &salt) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let user = User {
        id: 0,
        username: form.username.clone(),
        password_hash: password_hash.to_string(),
        created_at: timestamp,
    };

    match user.insert(&db).await {
        Ok(user_id) => {
            let cookie =
                auth::create_session(&db, user_id, timestamp, addr.to_string(), user_agent).await;
            ([("HX-Redirect", "/")], jar.add(cookie)).into_response()
        }
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            signup_form::build_with_error_message(&form.username, "Username already taken", "")
                .into_response()
        }
        Err(err) => {
            warn!("internal server error {err}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

fn validate_username(username: &str) -> &'static str {
    if username.len() < 5 || username.len() > 20 || !username.chars().all(char::is_alphanumeric) {
        "Username must be between 5 and 20 characters, and only contain letters / numbers."
    } else {
        ""
    }
}

const fn validate_password(password: &str) -> &'static str {
    if password.len() < 8 || password.len() > 40 || !password.is_ascii() {
        "Password must be between 8 and 40 characters and only contain ascii characters."
    } else {
        ""
    }
}

use argon2::PasswordHasher;
use argon2::{password_hash::SaltString, Argon2};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::CookieJar;
use rand::rngs::OsRng;
use serde::Deserialize;
use tracing::warn;

use crate::app_state::timestamp_micros;
use crate::{
    app_state::AppState,
    models::user_record,
    templates::{base, navbar, signup},
};

pub async fn get(state: State<AppState>, jar: CookieJar) -> impl IntoResponse {
    match state.authenticate(jar).await {
        Some(_) => Redirect::to("/").into_response(),
        None => Html(base(&navbar::build(), &signup::build())).into_response(),
    }
}

#[derive(Deserialize, Debug)]
pub struct Credentials {
    username: String,
    password: String,
}

pub async fn post(
    state: State<AppState>,
    jar: CookieJar,
    Form(form): Form<Credentials>,
) -> impl IntoResponse {
    let timestamp = timestamp_micros();

    if !validate_username(&form.username) {
        return signup::build_with_error_message("Invalid username").into_response();
    }
    if !validate_password(&form.password) {
        return signup::build_with_error_message("Invalid password").into_response();
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let Ok(password_hash) = argon2.hash_password(&form.password.as_bytes(), &salt) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    match user_record::insert(
        &state.database,
        &form.username,
        &password_hash.to_string(),
        timestamp,
    )
    .await
    {
        Ok(user_id) => {
            let cookie = state.make_auth_session(user_id).await;
            ([("HX-Redirect", "/")], jar.add(cookie)).into_response()
        }
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            signup::build_with_error_message("Username already taken").into_response()
        }
        Err(err) => {
            warn!("internal server error {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

fn validate_username(username: &str) -> bool {
    username.len() >= 3 && username.len() <= 20 && username.chars().all(char::is_alphanumeric)
}

fn validate_password(password: &str) -> bool {
    password.len() >= 8 && password.len() <= 100 && password.is_ascii()
}

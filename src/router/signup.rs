use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use tracing::warn;

use crate::{
    app_state::AppState,
    models::user,
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
    let timestamp = 100;

    match user::insert(&state.database, &form.username, &form.password, timestamp).await {
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

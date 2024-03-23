use std::net::SocketAddr;

use axum::{
    extract::ConnectInfo,
    response::{Html, IntoResponse, Redirect},
    Extension, Form,
};
use axum_extra::{
    extract::{cookie::Cookie, CookieJar},
    headers::UserAgent,
    TypedHeader,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use tracing::info;

use crate::{
    app_state::timestamp_micros,
    auth::{self, UserExtractor},
    models::session::Session,
    templates::{login, login_form},
};

pub async fn get(UserExtractor(user): UserExtractor) -> impl IntoResponse {
    match user {
        Some(_) => Redirect::to("/").into_response(),
        None => Html(login::build()).into_response(),
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

    match auth::login(
        &db,
        &form.username,
        &form.password,
        timestamp,
        addr.ip().to_string(),
        user_agent,
    )
    .await
    {
        Some(cookie) => {
            info!("User {} logged in", form.username);
            ([("HX-Redirect", "/")], jar.add(cookie)).into_response()
        }
        None => login_form::build("Incorrect username / password combination").into_response(),
    }
}

pub async fn delete(jar: CookieJar, db: Extension<SqlitePool>) -> impl IntoResponse {
    if let Some(cookie) = jar.get("session_id") {
        Session::delete_by_id(&db, cookie.value())
            .await
            .expect("failed to delete session id from database");
    }
    (
        [("HX-Redirect", "/")],
        jar.remove(Cookie::build("session_id")),
    )
        .into_response()
}

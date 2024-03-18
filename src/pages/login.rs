use axum::{
    response::{Html, IntoResponse, Redirect},
    Extension, Form,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::Deserialize;
use sqlx::SqlitePool;
use tracing::info;

use crate::{
    auth::{self, UserExtractor},
    templates::login,
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
    db: Extension<SqlitePool>,
    Form(form): Form<Credentials>,
) -> impl IntoResponse {
    match auth::login(&db, &form.username, &form.password).await {
        Some(cookie) => {
            info!("User {} logged in", form.username);
            ([("HX-Redirect", "/")], jar.add(cookie)).into_response()
        }
        None => todo!(), /*login::build_with_error_message("Incorrect username / password combination")
                         .into_response(),*/
    }
}

pub async fn delete(jar: CookieJar, db: Extension<SqlitePool>) -> impl IntoResponse {
    if let Some(cookie) = jar.get("session_id") {
        auth::logout(&db, cookie.value()).await;
    }
    (
        [("HX-Redirect", "/")],
        jar.remove(Cookie::build("session_id")),
    )
        .into_response()
}

use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::Deserialize;

use crate::{
    app_state::AppState,
    templates::{base, login, navbar},
};

pub async fn get(state: State<AppState>, jar: CookieJar) -> impl IntoResponse {
    match state.authenticate(jar).await {
        Some(_) => Redirect::to("/").into_response(),
        None => Html(base(&navbar::build(), &login::build())).into_response(),
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
    match state.login(&form.username, &form.password).await {
        Some(cookie) => ([("HX-Redirect", "/")], jar.add(cookie)).into_response(),
        None => login::build_with_error_message("Incorrect username / password combination")
            .into_response(),
    }
}

pub async fn delete(state: State<AppState>, jar: CookieJar) -> impl IntoResponse {
    if let Some(cookie) = jar.get("session_id") {
        state.logout(cookie.value()).await;
    }
    (
        [("HX-Redirect", "/")],
        jar.remove(Cookie::build("session_id")),
    )
        .into_response()
}

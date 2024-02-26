use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::extract::CookieJar;

use crate::{
    templates::{about, base, navbar},
    AppState,
};

pub async fn get(state: State<AppState>, jar: CookieJar) -> impl IntoResponse {
    match state.authenticate(jar).await {
        Some(user) => Html(base(
            &navbar::build_with_username(&user.username),
            &about::build(),
        ))
        .into_response(),
        None => Html(base(&navbar::build(), &about::build())).into_response(),
    }
}

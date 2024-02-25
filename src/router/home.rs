use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::extract::CookieJar;

use crate::{
    templates::{base, home, navbar},
    AppState,
};

pub async fn get(state: State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let user = state.authenticate(jar).await;

    match user {
        Some(user) => Html(base(
            &navbar::build_with_username(&user.username),
            &home::build(),
        ))
        .into_response(),
        None => Html(base(&navbar::build(), &home::build())).into_response(),
    }
}

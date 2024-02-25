use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::extract::CookieJar;

use crate::{templates::home, AppState};

pub async fn get(state: State<AppState>, jar: CookieJar) -> impl IntoResponse {
    Html(home::build()).into_response()
}

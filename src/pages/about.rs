use crate::{auth::UserExtractor, templates::about};
use axum::response::{Html, IntoResponse};

pub async fn get(UserExtractor(user): UserExtractor) -> impl IntoResponse {
    match user {
        Some(user) => Html(about::build(&user.username)),
        None => Html(about::build("")),
    }
}

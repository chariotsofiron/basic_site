use axum::response::{Html, IntoResponse};

use crate::{auth::UserExtractor, templates::home};

pub async fn get(UserExtractor(user): UserExtractor) -> impl IntoResponse {
    match user {
        Some(user) => Html(home::build(&user.username)).into_response(),
        None => Html(home::build("")).into_response(),
    }
}

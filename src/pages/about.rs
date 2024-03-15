use crate::{
    auth::UserExtractor,
    templates::{about, base, navbar},
};
use axum::response::{Html, IntoResponse};

pub async fn get(UserExtractor(user): UserExtractor) -> impl IntoResponse {
    match user {
        Some(user) => Html(base(
            &navbar::build_with_username(&user.username),
            &about::build(),
        ))
        .into_response(),
        None => Html(base(&navbar::build(), &about::build())).into_response(),
    }
}

use axum::response::{Html, IntoResponse};

use crate::{
    auth::UserExtractor,
    templates::{base, home, navbar},
};

pub async fn get(UserExtractor(user): UserExtractor) -> impl IntoResponse {
    match user {
        Some(user) => Html(base(
            &navbar::build_with_username(&user.username),
            &home::build(&user.username),
        ))
        .into_response(),
        None => Html(base(&navbar::build(), &home::build("world"))).into_response(),
    }
}

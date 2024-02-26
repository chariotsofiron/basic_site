use crate::app_state::AppState;
use axum::routing::get;
use axum::Router;

mod about;
mod home;
mod session;
mod signup;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(home::get))
        .route("/about", get(about::get))
        .route(
            "/login",
            get(session::get)
                .post(session::post)
                .delete(session::delete),
        )
        .route("/signup", get(signup::get).post(signup::post))
        .with_state(state)
}

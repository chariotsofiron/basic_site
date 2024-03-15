use crate::app_state::AppState;
use axum::routing::get;
use axum::Router;

mod about;
mod home;
mod login;
mod signup;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(home::get))
        .route("/about", get(about::get))
        .route(
            "/login",
            get(login::get).post(login::post).delete(login::delete),
        )
        .route("/signup", get(signup::get).post(signup::post))
        .with_state(state)
}

use crate::app_state::AppState;
use axum::routing::{delete, get};
use axum::Router;

mod about;
mod home;
mod login;
mod profile;
mod signup;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(home::get))
        .route("/about", get(about::get))
        .route("/profile", get(profile::get))
        .route(
            "/login",
            get(login::get).post(login::post).delete(login::delete),
        )
        .route("/login/:session_id", delete(login::delete_by_id))
        .route("/signup", get(signup::get).post(signup::post))
        .with_state(state)
}

use crate::app_state::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

mod home;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(home::get))
        .with_state(state)
}

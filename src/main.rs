#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod app_state;
mod auth;
mod models;
mod pages;
mod templates;

use app_state::AppState;
use axum::{http::header, response::IntoResponse, routing::get, Extension, Router};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

/// Connects to the database using the `DATABASE_URL` environment variable.
async fn connect_db() -> SqlitePool {
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set");
    SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to database")
}

async fn get_pico_css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css")],
        include_str!("../static/pico.min.css"),
    )
}
async fn get_pico_colors() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css")],
        include_str!("../static/pico.colors.css"),
    )
}
async fn get_htmx() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript")],
        include_str!("../static/htmx.min.js"),
    )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let pool = connect_db().await;
    let state = AppState::default();
    let app = Router::new()
        .route("/pico.min.css", get(get_pico_css))
        .route("/pico.colors.min.css", get(get_pico_colors))
        .route("/htmx.min.js", get(get_htmx))
        .nest("/", pages::router(state))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    info!("Starting server on {addr}");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Failed to start server");
}

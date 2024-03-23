#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod app_state;
mod auth;
mod models;
mod pages;
mod templates;

use app_state::AppState;
use axum::{Extension, Router};
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let pool = connect_db().await;
    let state = AppState::default();
    let app = Router::new()
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

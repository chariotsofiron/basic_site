#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod app_state;
mod models;
mod router;
mod templates;

use app_state::AppState;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let state = AppState::build().await;
    let app = Router::new().nest("/", router::router(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    info!("Starting server on {addr}");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

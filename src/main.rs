use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::{env, net::SocketAddr};

#[derive(Serialize)]
struct Message {
    message: &'static str,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn hello() -> Json<Message> {
    Json(Message { message: "Hello from Rust on Vercel" })
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(80);
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let mut terminate =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler");

    terminate.recv().await;
}

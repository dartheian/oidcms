mod extractor;
mod handler;
mod parameter;

use axum::routing::post;
use axum::{routing::get, Router};
use handler::authorize::authorize;
use handler::token::token;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

#[tokio::main]
async fn main() {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));
    let app = Router::new()
        .route("/authorize", get(authorize))
        .route("/token", post(token))
        .layer(session_layer);
    let address = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

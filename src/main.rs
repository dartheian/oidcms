mod authorize;
mod parameter;
mod serde_utils;
mod state;
mod token;

use authorize::handler::authorize;
use axum::routing::post;
use axum::{routing::get, Router};
use state::AppState;
use std::net::SocketAddr;
use token::handler::token;
use tokio::net::TcpListener;
use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};

async fn shutdown_signal() {
    let ctrl_c = async {
        ctrl_c().await.unwrap();
    };
    let terminate = async {
        signal(SignalKind::terminate()).unwrap().recv().await;
    };
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    let state = AppState::default();
    let app = Router::new()
        .route("/authorize", get(authorize))
        .route("/token", post(token))
        .with_state(state);
    let address = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

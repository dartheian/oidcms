mod authorize;
mod bounded_string;
mod config;
mod parameter;
mod state;
mod token;

use authorize::handler::authorize;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{routing::get, Router};
use config::Configuration;
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
    let config = Configuration::new();
    let address: SocketAddr = (config.host, config.port).into();
    let state = AppState::from(config);
    let app = Router::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .route("/authorize", get(authorize))
        .route("/token", post(token))
        .with_state(state);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

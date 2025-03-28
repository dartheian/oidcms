mod authorize;
mod bounded_string;
mod config;
mod crypto;
mod data;
mod state;
mod token;
mod userinfo;

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
use userinfo::userinfo;

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
    let address: SocketAddr = ([0, 0, 0, 0], config.port).into();
    let state = AppState::from(config);
    let router = Router::new()
        .route("/authorize", get(authorize))
        .route("/health", get(|| async { StatusCode::OK }))
        .route("/token", post(token))
        .route("/userinfo", get(userinfo))
        .with_state(state);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

mod authorize;
mod parameter;
mod serde_utils;
mod session;
mod token;

use authorize::handler::authorize;
use axum::routing::post;
use axum::{routing::get, Router};
use session::Session;
use std::net::SocketAddr;
use token::handler::token;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = Session::default();
    let app = Router::new()
        .route("/authorize", get(authorize))
        .route("/token", post(token))
        .with_state(state);
    let address = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

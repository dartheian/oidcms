use crate::state::User;
use crate::{bounded_string::SecureString, data::Secret};
use axum::http::Uri;
use config::{Config, Environment};
use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Deserialize)]
pub struct Configuration {
    #[serde(with = "http_serde::uri")]
    pub audience: Uri,
    pub client_secret: SecureString,
    pub expiration: u64,
    pub host: Ipv4Addr,
    #[serde(with = "http_serde::uri")]
    pub issuer: Uri,
    pub port: u16,
    pub rng_seed: u64,
    pub secret: Secret,
    pub user: User,
}

impl Configuration {
    pub fn new() -> Self {
        let env = Environment::default().separator("__");
        Config::builder()
            .add_source(env)
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}

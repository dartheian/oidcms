use crate::data::Secret;
use axum::http::Uri;
use config::{Config, Environment};
use serde::Deserialize;
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Configuration {
    pub expiration: u64,
    pub host: Ipv4Addr,
    #[serde(with = "http_serde::uri")]
    pub issuer: Uri,
    pub port: u16,
    pub rng_seed: u64,
    pub secret: Secret,
    pub user_file: PathBuf,
}

impl Configuration {
    pub fn new() -> Self {
        Config::builder()
            .add_source(Environment::default())
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}

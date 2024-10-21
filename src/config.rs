use std::net::Ipv4Addr;

use axum::http::Uri;
use config::{Config, Environment};
use serde::Deserialize;
use serde_with::base64::{Base64, Standard};
use serde_with::formats::Padded;
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize)]
pub struct Configuration {
    pub expiration: u64,
    pub host: Ipv4Addr,
    #[serde(with = "http_serde::uri")]
    pub issuer: Uri,
    pub port: u16,
    pub rng_seed: u64,
    #[serde_as(as = "Base64<Standard, Padded>")]
    pub secret: Vec<u8>,
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

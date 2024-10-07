use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
#[serde(transparent)]
struct UnixTime(u64);

impl UnixTime {
    pub fn new() -> Self {
        Self(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
    }
}

#[derive(Serialize)]
#[serde(transparent)]
pub struct Expiration(UnixTime);

impl Expiration {
    pub fn new() -> Self {
        Self(UnixTime::new())
    }
}

#[derive(Serialize)]
#[serde(transparent)]
pub struct IssuedAt(UnixTime);

impl IssuedAt {
    pub fn new() -> Self {
        Self(UnixTime::new())
    }
}

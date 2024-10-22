use serde::Serialize;
use std::{
    ops::Add,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize)]
#[serde(transparent)]
struct UnixTime(u64);

impl UnixTime {
    pub fn new() -> Self {
        Self(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }
}

impl Add<u64> for UnixTime {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

#[derive(Serialize)]
#[serde(transparent)]
pub struct Expiration(UnixTime);

impl Expiration {
    pub fn new(seconds: u64) -> Self {
        Self(UnixTime::new() + seconds)
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

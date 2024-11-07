use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UnixTime(u64);

impl UnixTime {
    pub fn now() -> Self {
        Self(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }
    pub fn expired(&self) -> bool {
        let now = UnixTime::now();
        self.0 <= now.0
    }
}

impl Add<u64> for UnixTime {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

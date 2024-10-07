use crate::serde_utils::bounded_string::LowerBoundedString;
use derive_more::derive::{AsRef, Display};
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;

const MIN_LEN: usize = 20;

#[derive(AsRef, Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq)]
#[as_ref(forward)]
#[serde(transparent)]
pub struct Code(LowerBoundedString<MIN_LEN>);

impl Code {
    pub fn new() -> Self {
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), MIN_LEN);
        Self(string.as_str().try_into().unwrap())
    }
}

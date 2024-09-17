use crate::serde_utils::bounded_string::UpperBoundedString;
use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;

const LEN: usize = 20;

#[derive(Clone, Serialize)]
pub struct Subject(UpperBoundedString<255>);

impl Subject {
    pub fn new() -> Self {
        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), LEN);
        Self(random_string.as_str().try_into().unwrap())
    }
}

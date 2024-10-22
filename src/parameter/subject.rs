use crate::bounded_string::{BoundedString, UpperBoundedString};
use crate::state::FromRng;
use rand::Rng;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Subject(UpperBoundedString<255>);

impl FromRng for Subject {
    fn from_rng<R: Rng>(rng: &mut R) -> Self {
        Self(BoundedString::random(rng, 20))
    }
}

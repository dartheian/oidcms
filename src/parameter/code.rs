use crate::bounded_string::{BoundedString, LowerBoundedString};
use crate::state::FromRng;
use derive_more::derive::{AsRef, Display};
use rand::Rng;
use serde::Deserialize;

#[derive(AsRef, Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq)]
#[as_ref(forward)]
#[serde(transparent)]
pub struct Code(LowerBoundedString<20>);

impl FromRng for Code {
    fn from_rng<R: Rng>(rng: &mut R) -> Self {
        Self(BoundedString::random(rng, 20))
    }
}

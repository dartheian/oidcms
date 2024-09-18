use derive_more::derive::AsRef;
use derive_more::Display;
use serde::Deserialize;
use thiserror::Error;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("state string must be at least 20 characters long")]
    TooShort,
}

#[derive(AsRef, Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq)]
#[as_ref(forward)]
#[serde(try_from = "String")]
pub struct State(String);

impl TryFrom<String> for State {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() >= 20 {
            Ok(State(value))
        } else {
            Err(Self::Error::TooShort)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_enough() {
        let string = "a".repeat(20);
        assert_eq!(State::try_from(string.clone()), Ok(State(string)));
    }

    #[test]
    fn too_short() {
        let string = "a".repeat(19);
        assert_eq!(State::try_from(string), Err(ParseError::TooShort));
    }
}

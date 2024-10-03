use derive_more::derive::AsRef;
use derive_more::Display;
use serde::{Deserialize, Deserializer};
use thiserror::Error;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("state string must be at least 20 characters long")]
    TooShort,
}

#[derive(AsRef, Clone, Debug, Display, Eq, Hash, PartialEq)]
#[as_ref(forward)]
pub struct State(String);

fn validate(value: &str) -> Result<(), ParseError> {
    if value.len() >= 20 {
        Ok(())
    } else {
        Err(ParseError::TooShort)
    }
}

impl<'de> Deserialize<'de> for State {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let string = String::deserialize(d)?;
        validate(&string).map_err(serde::de::Error::custom)?;
        Ok(Self(string))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_enough() {
        let string = "a".repeat(20);
        assert!(validate(&string).is_ok());
    }

    #[test]
    fn too_short() {
        let string = "a".repeat(19);
        assert_eq!(validate(&string), Err(ParseError::TooShort));
    }
}

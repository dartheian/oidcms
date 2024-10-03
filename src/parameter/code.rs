use derive_more::derive::{AsRef, Display};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserializer, Serialize};
use thiserror::Error;

const MIN_LEN: usize = 20;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("code string must be at least 20 characters long")]
    TooShort,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(AsRef, Clone, Debug, Display, Serialize)]
#[as_ref(forward)]
#[serde(transparent)]
pub struct Code(String);

fn validate(value: &str) -> Result<(), ParseError> {
    if value.len() < MIN_LEN {
        Err(ParseError::TooShort)
    } else {
        Ok(())
    }
}

impl Code {
    pub fn new() -> Self {
        Self(Alphanumeric.sample_string(&mut rand::thread_rng(), MIN_LEN))
    }
}

impl<'de> serde::Deserialize<'de> for Code {
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
        let string = "QnowT-aeawtOJKp-MtkH&".to_string();
        assert!(validate(&string).is_ok());
    }

    #[test]
    fn too_short() {
        assert_eq!(validate(""), Err(ParseError::TooShort));
    }
}

use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;
use thiserror::Error;

const LEN: usize = 20;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("subject string must not be empty")]
    Empty,
    #[error("subject string must be no more than 255 characters long")]
    TooLong,
}

#[derive(Clone, Serialize)]
pub struct Subject(String);

fn validate(value: &str) -> Result<(), ParseError> {
    if value.is_empty() {
        return Err(ParseError::Empty);
    }
    if 255 < value.len() {
        return Err(ParseError::TooLong);
    }
    Ok(())
}

impl Subject {
    pub fn new() -> Self {
        Self(Alphanumeric.sample_string(&mut rand::thread_rng(), LEN))
    }
}

impl TryFrom<String> for Subject {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_enough() {
        let string = "a".repeat(255);
        assert!(validate(&string).is_ok());
    }

    #[test]
    fn empty() {
        assert_eq!(validate(""), Err(ParseError::Empty));
    }

    #[test]
    fn too_short() {
        let string = "a".repeat(256);
        assert_eq!(validate(&string), Err(ParseError::TooLong));
    }
}

use serde::{Deserialize, Deserializer, Serialize};
use thiserror::Error;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("empty string")]
    Empty,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct ClientId(String);

fn validate(value: &str) -> Result<(), ParseError> {
    if value.is_empty() {
        Err(ParseError::Empty)
    } else {
        Ok(())
    }
}

impl<'de> Deserialize<'de> for ClientId {
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
        let string = "uAaunofWkaDJxukCFeBx".to_string();
        assert!(validate(&string).is_ok());
    }

    #[test]
    fn empty() {
        assert_eq!(validate(""), Err(ParseError::Empty));
    }
}

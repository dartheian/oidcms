use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("empty string")]
    Empty,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct ClientId(String);

impl TryFrom<String> for ClientId {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ParseError::Empty)
        } else {
            Ok(Self(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_enough() {
        let string = "client_id".to_string();
        assert_eq!(ClientId::try_from(string.clone()), Ok(ClientId(string)));
    }

    #[test]
    fn empty() {
        assert_eq!(ClientId::try_from("".to_string()), Err(ParseError::Empty));
    }
}

use crate::crypto::SECURE_LENGTH;
use derive_more::derive::{AsRef, Display};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(AsRef, Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[as_ref(forward)]
#[serde(try_from = "String")]
pub struct BoundedString<const L: usize, const U: usize>(String);

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("empty string")]
    Empty,
    #[error("string too short, must be at least {0} characters long: found `{1}`")]
    TooShort(usize, String),
    #[error("string too long, must be at most {0} characters long: found `{1}`")]
    TooLong(usize, String),
}

impl<const L: usize, const U: usize> TryFrom<&str> for BoundedString<L, U> {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() && L > 0 {
            return Err(ParseError::Empty);
        }
        if value.len() < L {
            return Err(ParseError::TooShort(L, value.into()));
        }
        if value.len() > U {
            return Err(ParseError::TooLong(U, value.into()));
        }
        Ok(Self(value.into()))
    }
}

impl<const L: usize, const U: usize> TryFrom<String> for BoundedString<L, U> {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let s: &str = value.as_ref();
        s.try_into()
    }
}

pub type NonEmptyString = BoundedString<1, { usize::MAX }>;
pub type SecureString = BoundedString<SECURE_LENGTH, { usize::MAX }>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid() {
        let result = BoundedString::<2, 3>::try_from("123");
        assert!(result.is_ok())
    }

    #[test]
    fn empty() {
        let result = BoundedString::<2, 3>::try_from("");
        assert!(matches!(result, Err(ParseError::Empty)))
    }

    #[test]
    fn too_short() {
        let result = BoundedString::<2, 3>::try_from("1");
        assert!(matches!(result, Err(ParseError::TooShort(_, _))))
    }

    #[test]
    fn too_long() {
        let result = BoundedString::<2, 3>::try_from("1234");
        assert!(matches!(result, Err(ParseError::TooLong(_, _))))
    }

    #[test]
    fn edge_case() {
        let result = BoundedString::<3, 3>::try_from("123");
        assert!(result.is_ok())
    }
}

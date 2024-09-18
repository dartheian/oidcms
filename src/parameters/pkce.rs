use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::{DecodeError, Engine};
use derive_more::derive::AsRef;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("PKCE string must be at least 40 characters long")]
    TooShort,
    #[error("PKCE string must be no more than 128 characters long")]
    TooLong,
    #[error("PKCE string must be base64url encoded: {0}")]
    NotBase64Urlencoded(#[from] DecodeError),
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(AsRef, Clone, Debug, Deserialize, Serialize)]
#[as_ref(forward)]
#[serde(try_from = "String")]
struct PkceCode(String);

impl TryFrom<String> for PkceCode {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 40 {
            return Err(ParseError::TooShort);
        }
        if 128 < value.len() {
            return Err(ParseError::TooLong);
        }
        URL_SAFE_NO_PAD.decode(&value)?;
        Ok(Self(value))
    }
}

#[derive(AsRef, Clone, Debug, Deserialize, Serialize)]
#[as_ref(forward)]
#[serde(transparent)]
pub struct CodeChallenge(PkceCode);

#[derive(AsRef, Deserialize)]
#[as_ref(forward)]
#[serde(transparent)]
pub struct CodeVerifier(PkceCode);

pub fn verify(challenge: CodeChallenge, verifier: CodeVerifier) -> bool {
    let hashed_verifier = Sha256::digest(verifier);
    let hashed_verifier = URL_SAFE_NO_PAD.encode(hashed_verifier);
    hashed_verifier == challenge.as_ref()
}

#[cfg(test)]
mod test {
    use super::*;

    const CODE_CHALLENGE: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
    const CODE_VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";

    mod parse {
        use super::*;

        #[test]
        fn valid() {
            assert_eq!(
                Ok(PkceCode(CODE_CHALLENGE.to_string())),
                PkceCode::try_from(CODE_CHALLENGE.to_string())
            )
        }

        #[test]
        fn too_short() {
            let string = "a".repeat(39);
            assert_eq!(Err(ParseError::TooShort), PkceCode::try_from(string));
        }

        #[test]
        fn too_long() {
            let string = "a".repeat(129);
            assert_eq!(Err(ParseError::TooLong), PkceCode::try_from(string));
        }

        #[test]
        fn long_enough() {
            let string = "a".repeat(40);
            assert_eq!(Ok(PkceCode(string.clone())), PkceCode::try_from(string))
        }

        #[test]
        fn short_enough() {
            let string = "a".repeat(128);
            assert_eq!(Ok(PkceCode(string.clone())), PkceCode::try_from(string))
        }
    }

    mod verify {
        use super::*;

        #[test]
        fn valid() {
            let challenge = CodeChallenge(PkceCode(CODE_CHALLENGE.to_string()));
            let verifier = CodeVerifier(PkceCode(CODE_VERIFIER.to_string()));
            assert_eq!(true, verify(challenge, verifier));
        }

        #[test]
        fn invalid() {
            let challenge = CodeChallenge(PkceCode(CODE_CHALLENGE.to_string()));
            let verifier = CodeVerifier(PkceCode("a".repeat(43)));
            assert_eq!(false, verify(challenge, verifier));
        }
    }
}

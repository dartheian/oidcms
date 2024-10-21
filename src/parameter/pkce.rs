use crate::bounded_string::{self, BoundedString};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::{DecodeError, Engine};
use derive_more::derive::{AsRef, Display};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("string must be base64url encoded: {0}")]
    NotBase64Urlencoded(#[from] DecodeError),
    #[error(transparent)]
    InvalidLength(#[from] bounded_string::ParseError),
}

#[derive(AsRef, Clone, Debug, Deserialize, Display)]
#[as_ref(forward)]
#[serde(try_from = "&str")]
struct PkceCode(BoundedString<40, 128>);

impl TryFrom<&str> for PkceCode {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        URL_SAFE_NO_PAD.decode(value)?;
        Ok(Self(value.try_into()?))
    }
}

#[derive(AsRef, Clone, Debug, Deserialize, Display)]
#[as_ref(forward)]
#[serde(transparent)]
pub struct CodeChallenge(PkceCode);

#[derive(AsRef, Debug, Deserialize, Display)]
#[as_ref(forward)]
#[serde(transparent)]
pub struct CodeVerifier(PkceCode);

pub fn verify(challenge: &CodeChallenge, verifier: &CodeVerifier) -> bool {
    let hashed_verifier = Sha256::digest(verifier);
    let hashed_verifier = URL_SAFE_NO_PAD.encode(hashed_verifier);
    hashed_verifier == challenge.as_ref()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Not;

    const CODE_CHALLENGE: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
    const CODE_VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";

    #[test]
    fn valid() {
        let challenge = CodeChallenge(CODE_CHALLENGE.try_into().unwrap());
        let verifier = CodeVerifier(CODE_VERIFIER.try_into().unwrap());
        assert!(verify(&challenge, &verifier));
    }

    #[test]
    fn invalid() {
        let challenge = CodeChallenge(CODE_CHALLENGE.try_into().unwrap());
        let verifier = CodeVerifier("a".repeat(40).as_str().try_into().unwrap());
        assert!(verify(&challenge, &verifier).not());
    }
}

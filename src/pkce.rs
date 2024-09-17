use anyhow::{anyhow, Context, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize, PartialEq)]
pub struct PkceCode(String);

impl PkceCode {
    pub fn verify(&self, challenge: &Self) -> bool {
        let converted_verif = Sha256::digest(&self.0);
        let b64_verif = URL_SAFE_NO_PAD.encode(converted_verif);
        b64_verif == challenge.0
    }
}

impl TryFrom<&str> for PkceCode {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        if value.len() <= 40 {
            return Err(anyhow!("PKCE code is too short"));
        }
        if 128 <= value.len() {
            return Err(anyhow!("PKCE code is too long"));
        }
        let _code = URL_SAFE_NO_PAD
            .decode(&value)
            .context("PKCE code is not base64url endcoded")?;
        Ok(Self(value.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE_CHALLENGE: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
    const CODE_VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";

    #[test]
    fn it_works() {
        let code = PkceCode::try_from(CODE_CHALLENGE);
        assert!(code.is_ok(), "Error while decoding code_challenge");
        assert_eq!(PkceCode(CODE_CHALLENGE.to_string()), code.unwrap());
    }

    #[test]
    fn short_code() {
        let code = "abcd";
        let code: anyhow::Result<PkceCode> = code.try_into();
        assert!(code.is_err(), "Should be too short: {:?}", &code);
    }

    #[test]
    fn too_long_code() {
        let code = "a".repeat(200);
        let code: &str = code.as_ref();
        let result: anyhow::Result<PkceCode> = code.try_into();
        assert!(result.is_err(), "Should be too long: {:?}", &code);
    }

    #[cfg(test)]
    mod verification {
        use crate::pkce::tests::CODE_CHALLENGE;

        use super::{PkceCode, CODE_VERIFIER};

        #[test]
        fn good_case() {
            let verifier_code = PkceCode(CODE_VERIFIER.into());
            let challenge_code = PkceCode(CODE_CHALLENGE.into());
            assert!(verifier_code.verify(&challenge_code), "verification failed",);
        }

        #[test]
        fn bad_case() {
            let verifier_code = PkceCode("some_random_string".into());
            let challenge_code = PkceCode(CODE_CHALLENGE.into());
            assert!(
                !verifier_code.verify(&challenge_code),
                "verification succeded, but should have failed",
            );
        }
    }
}

use anyhow::{anyhow, Context, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct PkceCode(Vec<u8>);

impl TryFrom<&str> for PkceCode {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        if value.len() <= 40 {
            return Err(anyhow!("PKCE code is too short"));
        }
        if 128 <= value.len() {
            return Err(anyhow!("PKCE code is too long"));
        }
        let code = URL_SAFE_NO_PAD
            .decode(&value)
            .context("PKCE code is not base64url endcoded")?;
        Ok(Self(code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE_CHALLENGE: &str = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
    const CODE_VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";

    const RAW_CODE_CHALLENGE: [u8; 32] = [
        19, 211, 30, 150, 26, 26, 216, 236, 47, 22, 177, 12, 76, 152, 46, 8, 118, 168, 120, 173,
        109, 241, 68, 86, 110, 225, 137, 74, 203, 112, 249, 195,
    ];

    const RAW_CODE_VERIFIER: [u8; 32] = [
        116, 24, 223, 180, 151, 153, 224, 37, 79, 250, 96, 125, 216, 173, 187, 186, 22, 212, 37,
        77, 105, 214, 191, 240, 91, 88, 5, 88, 83, 132, 141, 121,
    ];

    #[test]
    fn it_works() {
        let code = PkceCode::try_from(CODE_CHALLENGE);
        assert!(code.is_ok(), "Error while decoding code_challenge");
        assert_eq!(PkceCode(RAW_CODE_CHALLENGE.into()), code.unwrap());
    }
}

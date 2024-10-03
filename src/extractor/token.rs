use crate::parameter::code::Code;
use crate::parameter::pkce::CodeVerifier;
use crate::parameter::GrantType;
use axum::extract::{Form, FromRequest};
use axum::http::Uri;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, FromRequest)]
#[from_request(via(Form))]
pub struct TokenParams {
    #[serde(deserialize_with = "code_verifier")]
    pub code_verifier: CodeVerifier,
    #[serde(deserialize_with = "code")]
    pub code: Code,
    #[serde(deserialize_with = "grant_type")]
    pub grant_type: GrantType,
    #[serde(deserialize_with = "redirect_uri")]
    pub redirect_uri: Uri,
}

fn code<'de, D>(deserializer: D) -> Result<Code, D::Error>
where
    D: Deserializer<'de>,
{
    Code::deserialize(deserializer)
        .map_err(|e| format!("error while parsing field `code`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn code_verifier<'de, D>(deserializer: D) -> Result<CodeVerifier, D::Error>
where
    D: Deserializer<'de>,
{
    CodeVerifier::deserialize(deserializer)
        .map_err(|e| format!("error while parsing field `code_verifier`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn grant_type<'de, D>(deserializer: D) -> Result<GrantType, D::Error>
where
    D: Deserializer<'de>,
{
    GrantType::deserialize(deserializer)
        .map_err(|e| format!("error while parsing field `grant_type`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn redirect_uri<'de, D>(deserializer: D) -> Result<Uri, D::Error>
where
    D: Deserializer<'de>,
{
    http_serde::uri::deserialize(deserializer)
        .map_err(|e| format!("error while parsing field `redirect_uri`: {e}"))
        .map_err(serde::de::Error::custom)
}

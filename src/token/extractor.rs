use crate::bounded_string::SecureString;
use crate::data::pkce::CodeVerifier;
use crate::data::GrantType;
use axum::extract::{Form, FromRequest};
use axum::http::Uri;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, FromRequest)]
#[from_request(via(Form))]
#[allow(unused)]
pub struct TokenParams {
    #[serde(deserialize_with = "client_id")]
    pub client_id: SecureString,
    #[serde(deserialize_with = "client_secret")]
    pub client_secret: SecureString,
    #[serde(deserialize_with = "code_verifier")]
    pub code_verifier: CodeVerifier,
    #[serde(deserialize_with = "code")]
    pub code: SecureString,
    #[serde(deserialize_with = "grant_type")]
    pub grant_type: GrantType,
    #[serde(deserialize_with = "redirect_uri")]
    pub redirect_uri: Uri,
}

fn client_id<'de, D: Deserializer<'de>>(d: D) -> Result<SecureString, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `client_id`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn client_secret<'de, D: Deserializer<'de>>(d: D) -> Result<SecureString, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `client_secret`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn code<'de, D: Deserializer<'de>>(d: D) -> Result<SecureString, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `code`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn code_verifier<'de, D: Deserializer<'de>>(d: D) -> Result<CodeVerifier, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `code_verifier`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn grant_type<'de, D: Deserializer<'de>>(d: D) -> Result<GrantType, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `grant_type`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn redirect_uri<'de, D: Deserializer<'de>>(d: D) -> Result<Uri, D::Error> {
    http_serde::uri::deserialize(d)
        .map_err(|e| format!("error while parsing field `redirect_uri`: {e}"))
        .map_err(serde::de::Error::custom)
}

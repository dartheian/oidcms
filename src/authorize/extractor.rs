use crate::bounded_string::SecureString;
use crate::data::pkce::CodeChallenge;
use crate::data::{CodeChallengeMethod, ResponseMode, ResponseType, Scope};
use axum::extract::{FromRequestParts, Query};
use axum::http::Uri;
use serde::{Deserialize, Deserializer};
use serde_with::formats::SpaceSeparator;
use serde_with::StringWithSeparator;
use serde_with::{serde_as, DeserializeAs};
use std::collections::HashSet;
use thiserror::Error;

#[serde_as]
#[derive(Clone, Deserialize, FromRequestParts)]
#[from_request(via(Query))]
pub struct AuthorizeParams {
    #[serde(deserialize_with = "client_id")]
    pub client_id: SecureString,
    #[serde(deserialize_with = "code_challenge_method")]
    pub code_challenge_method: CodeChallengeMethod,
    #[serde(deserialize_with = "code_challenge")]
    pub code_challenge: CodeChallenge,
    #[serde(deserialize_with = "redirect_uri")]
    pub redirect_uri: Uri,
    #[serde(deserialize_with = "response_mode")]
    pub response_mode: ResponseMode,
    #[serde(deserialize_with = "response_type")]
    pub response_type: ResponseType,
    #[serde(deserialize_with = "scope")]
    pub scope: HashSet<Scope>,
    #[serde(deserialize_with = "state")]
    pub state: SecureString,
}

fn client_id<'de, D: Deserializer<'de>>(d: D) -> Result<SecureString, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `client_id`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn code_challenge_method<'de, D: Deserializer<'de>>(d: D) -> Result<CodeChallengeMethod, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `code_challenge_method`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn code_challenge<'de, D: Deserializer<'de>>(d: D) -> Result<CodeChallenge, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `code_challenge`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn redirect_uri<'de, D: Deserializer<'de>>(d: D) -> Result<Uri, D::Error> {
    http_serde::uri::deserialize(d)
        .map_err(|e| format!("error while parsing field `redirect_uri`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn response_mode<'de, D: Deserializer<'de>>(d: D) -> Result<ResponseMode, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `response_mode`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn response_type<'de, D: Deserializer<'de>>(d: D) -> Result<ResponseType, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `response_type`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn scope<'de, D: Deserializer<'de>>(d: D) -> Result<HashSet<Scope>, D::Error> {
    StringWithSeparator::<SpaceSeparator, Scope>::deserialize_as(d)
        .and_then(|s| validate_scope(s).map_err(serde::de::Error::custom))
        .map_err(|e| format!("error while parsing field `scope`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn state<'de, D: Deserializer<'de>>(d: D) -> Result<SecureString, D::Error> {
    Deserialize::deserialize(d)
        .map_err(|e| format!("error while parsing field `state`: {e}"))
        .map_err(serde::de::Error::custom)
}

#[derive(Debug, Error)]
enum Error {
    #[error("missing scope `{0}`")]
    MissingScope(Scope),
}

fn validate_scope(set: HashSet<Scope>) -> Result<HashSet<Scope>, Error> {
    if set.contains(&Scope::Openid) {
        Ok(set)
    } else {
        Err(Error::MissingScope(Scope::Openid))
    }
}

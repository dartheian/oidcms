use crate::parameter::client_id::ClientId;
use crate::parameter::pkce::CodeChallenge;
use crate::parameter::scope::{Scope, ScopeSet};
use crate::parameter::state::State;
use crate::parameter::{CodeChallengeMethod, ResponseMode, ResponseType};
use axum::extract::{FromRequestParts, Query};
use axum::http::Uri;
use serde::{Deserialize, Deserializer};
use thiserror::Error;

#[derive(Deserialize, FromRequestParts)]
#[from_request(via(Query))]
pub struct AuthorizeParams {
    #[serde(deserialize_with = "client_id")]
    pub client_id: ClientId,
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
    pub scope: ScopeSet,
    #[serde(deserialize_with = "state")]
    pub state: State,
}

fn client_id<'de, D>(deserializer: D) -> Result<ClientId, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
        .map_err(|e| format!("error while parsing field `client_id`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn code_challenge_method<'de, D>(deserializer: D) -> Result<CodeChallengeMethod, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
        .map_err(|e| format!("error while parsing field `code_challenge_method`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn code_challenge<'de, D>(deserializer: D) -> Result<CodeChallenge, D::Error>
where
    D: Deserializer<'de>,
{
    CodeChallenge::deserialize(deserializer)
        .map_err(|e| format!("error while parsing field `code_challenge`: {e}"))
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

fn response_mode<'de, D>(deserializer: D) -> Result<ResponseMode, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
        .map_err(|e| format!("error while parsing field `response_mode`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn response_type<'de, D>(deserializer: D) -> Result<ResponseType, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
        .map_err(|e| format!("error while parsing field `response_type`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn scope<'de, D>(deserializer: D) -> Result<ScopeSet, D::Error>
where
    D: Deserializer<'de>,
{
    ScopeSet::deserialize(deserializer)
        .and_then(|s| validate_scope(s).map_err(serde::de::Error::custom))
        .map_err(|e| format!("error while parsing field `scope`: {e}"))
        .map_err(serde::de::Error::custom)
}

fn state<'de, D>(deserializer: D) -> Result<State, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
        .map_err(|e| format!("error while parsing field `state`: {e}"))
        .map_err(serde::de::Error::custom)
}

#[derive(Debug, Error)]
enum Error {
    #[error("scope `{0}` must be present")]
    MissingScope(Scope),
}

fn validate_scope(set: ScopeSet) -> Result<ScopeSet, Error> {
    if set.contains(&Scope::Openid) {
        Ok(set)
    } else {
        Err(Error::MissingScope(Scope::Openid))
    }
}

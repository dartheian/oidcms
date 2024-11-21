use super::time::UnixTime;
use super::Scope;
use crate::bounded_string::SecureString;
use crate::state::AppState;
use axum::http::Uri;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct AccessToken {
    #[serde(with = "http_serde::uri")]
    pub aud: Uri,
    pub auth_time: UnixTime,
    pub cid: SecureString,
    pub exp: UnixTime,
    pub iat: UnixTime,
    #[serde(with = "http_serde::uri")]
    pub iss: Uri,
    pub jti: SecureString,
    pub scp: HashSet<Scope>,
    pub sub: SecureString,
    pub uid: SecureString,
    pub ver: u32,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid issuer {0} expected {0}")]
    InvalidIssuer(Uri, Uri),
    #[error("expired token")]
    Expired,
    #[error("missing scope: {0}")]
    MissingScope(Scope),
    #[error("at least one of these scopes: {}", display(.0))]
    MissingScopes(HashSet<Scope>),
}

impl AccessToken {
    pub fn validate(&self, state: &AppState) -> Result<(), Error> {
        if self.iss != state.issuer() {
            return Err(Error::InvalidIssuer(self.iss.clone(), state.issuer()));
        }
        if self.exp.expired() {
            return Err(Error::Expired);
        }
        if !self.scp.contains(&Scope::Openid) {
            return Err(Error::MissingScope(Scope::Openid));
        }
        if self.scp.is_disjoint(&state.required_scopes()) {
            return Err(Error::MissingScopes(state.required_scopes()));
        }
        Ok(())
    }
}

fn display(scopes: &HashSet<Scope>) -> String {
    scopes
        .into_iter()
        .fold(String::new(), |a, b| a + b.to_string().as_str() + ", ")
}

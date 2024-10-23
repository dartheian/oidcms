pub mod code;
pub mod pkce;
pub mod subject;
pub mod time;

use crate::bounded_string::{LowerBoundedString, NonEmptyString};
use axum::http::Uri;
use derive_more::derive::{Display, FromStr};
use serde::{Deserialize, Serialize};
use subject::Subject;
use time::{Expiration, IssuedAt};

#[derive(Clone, Deserialize)]
pub enum CodeChallengeMethod {
    S256,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseMode {
    FormPost,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Code,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Bearer,
}

#[derive(Clone, Debug, Deserialize, Display, FromStr, Hash, PartialEq, Eq, Serialize)]
#[display("{_variant}")]
pub enum Scope {
    Address,
    Email,
    Groups,
    Openid,
    Phone,
    Profile,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ClientId(NonEmptyString);

#[derive(Clone, Deserialize, Display)]
#[serde(transparent)]
pub struct State(LowerBoundedString<20>);

#[derive(Serialize)]
pub struct AccessToken {
    pub aud: ClientId,
    pub exp: Expiration,
    pub iat: IssuedAt,
    #[serde(with = "http_serde::uri")]
    pub iss: Uri,
    pub sub: Subject,
}

#[derive(Serialize)]
pub struct IdToken {
    pub client_id: ClientId,
    pub exp: Expiration,
    pub iat: IssuedAt,
    #[serde(with = "http_serde::uri")]
    pub iss: Uri,
    pub sub: Subject,
}

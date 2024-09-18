pub mod client_id;
pub mod pkce;
pub mod state;

use serde::Deserialize;

#[derive(Deserialize)]
pub enum CodeChallengeMethod {
    S256,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseMode {
    FormPost,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Code,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Openid,
}

pub mod access_token;
pub mod client_id;
pub mod code;
pub mod id_token;
pub mod pkce;
pub mod scope;
pub mod state;
pub mod subject;
pub mod time;
pub mod user_id;

use serde::{Deserialize, Serialize};

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
pub enum GrantType {
    AuthorizationCode,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Bearer,
}

pub mod access_token;
pub mod pkce;
pub mod time;

use crate::bounded_string::SecureString;
use axum::http::Uri;
use derive_more::derive::{AsRef, Display, FromStr};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, Standard};
use serde_with::formats::Padded;
use serde_with::serde_as;
use time::UnixTime;

#[serde_as]
#[derive(AsRef, Clone, Deserialize)]
pub struct Secret(#[serde_as(as = "Base64<Standard, Padded>")] Vec<u8>);

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

#[derive(Display, Serialize)]
pub enum AuthenticationMethod {
    #[display("Password authentication")]
    Pwd,
    #[display("Proof of possession of a key")]
    Pop,
    #[display("One time password")]
    Otp,
    #[display("Fingerprint biometric")]
    Fpt,
    #[display("Retina scan biometric")]
    Eye,
    #[display("Voice biometric")]
    Vbm,
    #[display("Confirmation by telephone call")]
    Tel,
    #[display("Confirmation by SMS reply")]
    Sms,
    #[display("Knowledge based authentication")]
    Kba,
    #[display("Windows integrated authentication")]
    Wia,
    #[display("Multiple factor authentication")]
    Mfa,
}

#[derive(Serialize)]
pub struct IdToken {
    pub amr: Vec<AuthenticationMethod>,
    pub at_hash: String,
    pub aud: SecureString,
    pub auth_time: UnixTime,
    pub exp: UnixTime,
    pub iat: UnixTime,
    #[serde(with = "http_serde::uri")]
    pub iss: Uri,
    pub jti: SecureString,
    pub sub: SecureString,
    pub ver: u32,
}

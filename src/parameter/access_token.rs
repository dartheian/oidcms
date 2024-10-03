use super::client_id::ClientId;
use super::subject::Subject;
use super::time::{Expiration, IssuedAt};
use axum::http::Uri;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

const SECRET: &[u8] = b"secret";

#[derive(Serialize)]
pub struct AccessTokenPayload {
    pub aud: ClientId,
    pub exp: Expiration,
    pub iat: IssuedAt,
    #[serde(with = "http_serde::uri")]
    pub iss: Uri,
    pub sub: Subject,
}

#[derive(Serialize)]
#[serde(transparent)]
pub struct AccessToken(String);

impl TryFrom<AccessTokenPayload> for AccessToken {
    type Error = jsonwebtoken::errors::Error;
    fn try_from(value: AccessTokenPayload) -> Result<Self, Self::Error> {
        let jwt = encode(
            &Header::default(),
            &value,
            &EncodingKey::from_secret(SECRET),
        )?;
        Ok(Self(jwt))
    }
}

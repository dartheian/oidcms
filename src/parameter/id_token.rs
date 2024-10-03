use super::client_id::ClientId;
use super::subject::Subject;
use super::time::{Expiration, IssuedAt};
use axum::http::Uri;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

const SECRET: &[u8] = b"secret";

#[derive(Serialize)]
pub struct IdTokenPayload {
    pub client_id: ClientId,
    pub exp: Expiration,
    pub iat: IssuedAt,
    #[serde(with = "http_serde::uri")]
    pub iss: Uri,
    pub sub: Subject,
}

#[derive(Serialize)]
#[serde(transparent)]
pub struct IdToken(String);

impl TryFrom<IdTokenPayload> for IdToken {
    type Error = jsonwebtoken::errors::Error;
    fn try_from(value: IdTokenPayload) -> Result<Self, Self::Error> {
        let jwt = encode(
            &Header::default(),
            &value,
            &EncodingKey::from_secret(SECRET),
        )?;
        Ok(Self(jwt))
    }
}

use crate::data::Secret;
use jsonwebtoken::errors::Result;
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;

pub fn encode<C: Serialize>(claims: C, secret: Secret) -> Result<String> {
    let header = &Header::default();
    let key = EncodingKey::from_secret(secret.as_ref());
    jsonwebtoken::encode(header, &claims, &key)
}

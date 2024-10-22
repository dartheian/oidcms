use jsonwebtoken::{errors::Result, EncodingKey, Header};
use serde::Serialize;

pub fn encode<C: Serialize>(claims: C, secret: Vec<u8>) -> Result<String> {
    let header = &Header::default();
    let key = EncodingKey::from_secret(&secret);
    jsonwebtoken::encode(header, &claims, &key)
}

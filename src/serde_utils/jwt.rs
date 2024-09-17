use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Serializer};

const SECRET: &[u8] = b"secret";

pub fn serialize<V: Serialize, S: Serializer>(v: &V, s: S) -> Result<S::Ok, S::Error> {
    let string = encode(&Header::default(), &v, &EncodingKey::from_secret(SECRET)).unwrap();
    s.serialize_str(&string)
}

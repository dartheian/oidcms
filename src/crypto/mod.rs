use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rand::{CryptoRng, Rng};

// https://datatracker.ietf.org/doc/html/rfc6749#section-10.10
pub const SECURE_LENGTH: usize = 20;

pub fn strong_random_bytes<R: Rng + CryptoRng>(mut rng: R) -> String {
    STANDARD.encode(rng.gen::<[u8; SECURE_LENGTH]>())
}

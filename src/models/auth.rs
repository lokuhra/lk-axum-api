use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: u64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Passengers {
    pub email: String,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

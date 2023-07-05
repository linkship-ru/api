use std::{time::{SystemTime, UNIX_EPOCH}, env};

use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    iat: u128,
}

pub fn generate_jwt(username: String) -> String {
    let secret: String = env::var("SECRET").expect("SECRET not found in your .env");

    let timestamp: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap()
        .as_millis();

    let claims: Claims = Claims {
        username: username.to_owned(),
        iat: timestamp.to_owned(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}
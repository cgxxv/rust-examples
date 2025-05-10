#![allow(unused)]

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // 用户ID
    pub exp: usize,   // 过期时间
    pub role: String, // 用户角色
}

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry_hours: i64,
}

impl JwtConfig {
    pub fn new(secret: String, expiry_hours: i64) -> Self {
        Self {
            secret,
            expiry_hours,
        }
    }
}

pub fn generate_token(user_id: &str, role: &str, config: &JwtConfig) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(config.expiry_hours))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        role: role.to_owned(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )
    .unwrap()
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}

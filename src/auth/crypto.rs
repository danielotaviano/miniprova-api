use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    sub: i32,
}

pub fn encrypt_password(password: &str) -> Result<String, ServiceError> {
    let salt: [u8; 16] = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>()
        .as_bytes()
        .try_into()
        .unwrap();

    match bcrypt::hash_with_salt(password, 4, salt) {
        Ok(hashed_password) => Ok(hashed_password.to_string()),
        Err(_) => Err(ServiceError::InternalServerError),
    }
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, ServiceError> {
    match bcrypt::verify(password, hashed_password) {
        Ok(is_valid) => Ok(is_valid),
        Err(_) => Err(ServiceError::InternalServerError),
    }
}

pub fn generate_token(user_id: i32) -> Result<String, ServiceError> {
    let claims = Claims {
        sub: user_id,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| ServiceError::InternalServerError)?;

    Ok(token)
}

pub fn decode_token(token: &str) -> Result<i32, ServiceError> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| ServiceError::Unauthorized)?;

    Ok(token_data.claims.sub)
}

use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::errors::ServiceError;

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

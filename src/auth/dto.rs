use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;

#[derive(Debug, Deserialize)]
pub struct LoginUserInputDto {
    pub email: String,
    pub password: String,
}

impl LoginUserInputDto {
    pub fn validate(&self) -> Result<(), ServiceError> {
        if self.email.is_empty() {
            return Err(ServiceError::BadRequest("Email is required".into()));
        }
        if self.password.is_empty() {
            return Err(ServiceError::BadRequest("Password is required".into()));
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct LoginUserOutputDto {
    pub token: String,
}

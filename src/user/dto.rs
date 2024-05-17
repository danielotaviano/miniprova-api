use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserInputDto {
    pub name: String,
    pub email: String,
}

impl CreateUserInputDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name is required".to_string());
        }

        if self.email.is_empty() {
            return Err("Email is required".to_string());
        }

        Ok(())
    }
}

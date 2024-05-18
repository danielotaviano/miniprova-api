use serde::{Deserialize, Serialize};

use crate::role::enm::RoleEnum;

use super::model::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserInputDto {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl CreateUserInputDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name is required".to_string());
        }

        if self.email.is_empty() {
            return Err("Email is required".to_string());
        }

        if self.password.is_empty() {
            return Err("Password is required".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserOutputDto {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl From<User> for CreateUserOutputDto {
    fn from(user: User) -> Self {
        CreateUserOutputDto {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]

pub struct UserWithRolesOutputDto {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub roles: Vec<RoleEnum>,
}

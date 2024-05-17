use std::vec;

use crate::{auth::crypto, errors::ServiceError, role::model::RoleEnum, user::repository};

use super::dto::{CreateUserInputDto, CreateUserOutputDto};

pub fn create_user(user: CreateUserInputDto) -> Result<CreateUserOutputDto, ServiceError> {
    let existing_user = repository::get_user_by_email(&user.email)?;
    if existing_user.is_some() {
        return Err(ServiceError::BadRequest("Email already exists".into()));
    }

    let hashed_password = crypto::encrypt_password(&user.password)?;
    let user = repository::create_user(
        CreateUserInputDto {
            password: hashed_password,
            ..user
        },
        vec![RoleEnum::STUDENT],
    )?;

    Ok(user.into())
}

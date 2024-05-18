use std::vec;

use crate::{auth::crypto, errors::ServiceError, role::enm::RoleEnum, user::repository};

use super::dto::{CreateUserInputDto, CreateUserOutputDto, UserWithRolesOutputDto};

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

pub fn get_user_with_roles_by_id(
    user_id: i32,
) -> Result<Option<UserWithRolesOutputDto>, ServiceError> {
    let user = repository::get_user_with_roles_by_id(user_id)?;
    Ok(user.into())
}

pub fn set_user_roles(user_id: i32, roles: Vec<RoleEnum>) -> Result<(), ServiceError> {
    repository::set_user_roles(user_id, roles)?;
    Ok(())
}

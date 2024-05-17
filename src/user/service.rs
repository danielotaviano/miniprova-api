use crate::{errors::ServiceError, user::repository};

use super::{dto::CreateUserInputDto, model::User};

pub fn create_user(user: CreateUserInputDto) -> Result<User, ServiceError> {
    let existing_user = repository::get_user_by_email(&user.email)?;

    if existing_user.is_some() {
        return Err(ServiceError::BadRequest("Email already exists".into()));
    }

    let user = repository::create_user(user)?;

    Ok(user)
}

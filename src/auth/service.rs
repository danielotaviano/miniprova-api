use crate::{errors::ServiceError, user};

use super::{
    crypto,
    dto::{LoginUserInputDto, LoginUserOutputDto},
};

pub fn login(login: LoginUserInputDto) -> Result<LoginUserOutputDto, ServiceError> {
    let user = user::repository::get_user_by_email(&login.email)
        .map_err(|_| ServiceError::InternalServerError)?;
    if user.is_none() {
        return Err(ServiceError::BadRequest(
            "We couldn't find a user with these credentials".into(),
        ));
    }

    let user = user.unwrap();
    let is_valid = crypto::verify_password(&login.password, &user.password)?;
    if !is_valid {
        return Err(ServiceError::BadRequest(
            "We couldn't find a user with these credentials".into(),
        ));
    }

    let token = crypto::generate_token(user.id)?;
    Ok(LoginUserOutputDto { token })
}

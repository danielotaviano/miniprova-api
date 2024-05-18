use crate::errors::ServiceError;

use super::{dto::CreateAvatarInputDto, model::Avatar, repository};

pub fn create_avatar(
    avatar_user_id: i32,
    new_avatar: CreateAvatarInputDto,
) -> Result<Avatar, ServiceError> {
    let avatar = repository::create_avatar(avatar_user_id, new_avatar)?;
    Ok(avatar)
}

pub fn delete_avatar(avatar_user_id: i32) -> Result<(), ServiceError> {
    repository::delete_avatar(avatar_user_id)?;
    Ok(())
}

use diesel::{
    associations::HasTable, query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, Table,
};

use crate::{
    avatar::model::{Avatar, NewAvatar},
    db::DB_MANAGER,
    errors::ServiceError,
    schema::avatars::dsl::*,
};

use super::dto::CreateAvatarInputDto;

pub fn create_avatar(
    avatar_user_id: i32,
    new_avatar: CreateAvatarInputDto,
) -> Result<Avatar, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let new_avatar = NewAvatar {
        user_id: avatar_user_id,
        url: new_avatar.url,
    };

    let result_avatar: Avatar = diesel::insert_into(avatars::table())
        .values(&new_avatar)
        .on_conflict(user_id)
        .do_update()
        .set(&new_avatar)
        .returning(avatars::all_columns())
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(result_avatar)
}

pub fn delete_avatar(avatar_user_id: i32) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    diesel::delete(avatars.filter(user_id.eq(avatar_user_id)))
        .execute(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(())
}

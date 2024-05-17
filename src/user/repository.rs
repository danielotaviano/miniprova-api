use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, Table};

use super::dto::CreateUserInputDto;
use super::model::{NewUser, User};

use crate::db::DB_MANAGER;
use crate::errors::ServiceError;
use crate::schema::users::dsl::*;

pub fn create_user(user: CreateUserInputDto) -> Result<User, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let new_user = NewUser {
        name: &user.name,
        email: &user.email,
        created_at: chrono::Local::now().naive_local(),
    };

    let user = diesel::insert_into(users::table())
        .values(&new_user)
        .returning(users::all_columns())
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(user)
}

pub fn get_user_by_email(user_email: &str) -> Result<Option<User>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let user = users
        .filter(email.eq(user_email))
        .select(User::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(user)
}

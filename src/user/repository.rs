use std::error::Error;

use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, Table};

use super::dto::CreateUserInputDto;
use super::model::{NewUser, User};

use crate::db::DB_MANAGER;
use crate::errors::ServiceError;
use crate::role::model::{RoleEnum, UsersRole};
use crate::schema::users::dsl::*;
use crate::schema::users_roles::dsl::*;

pub fn create_user(user: CreateUserInputDto, roles: Vec<RoleEnum>) -> Result<User, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Result<User, Box<dyn Error>> = conn.transaction(|tx| {
        let new_user = NewUser {
            name: &user.name,
            email: &user.email,
            password: &user.password,
            created_at: chrono::Local::now().naive_local(),
        };

        let user: User = diesel::insert_into(users::table())
            .values(&new_user)
            .returning(users::all_columns())
            .get_result(tx)?;

        diesel::insert_into(users_roles::table())
            .values(
                roles
                    .into_iter()
                    .map(|role| UsersRole {
                        user_id: user.id,
                        role_name: role.into(),
                    })
                    .collect::<Vec<UsersRole>>(),
            )
            .execute(tx)?;

        Ok(user)
    });

    let user = result.map_err(|_| ServiceError::InternalServerError)?;

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

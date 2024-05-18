use std::error::Error;

use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, Table};

use super::dto::{CreateUserInputDto, UserWithRolesOutputDto};
use super::model::{NewUser, User, UsersRole};

use crate::db::DB_MANAGER;
use crate::errors::ServiceError;
use crate::role::enm::RoleEnum;
use crate::role::model::Role;
use crate::schema::users::dsl::*;
use crate::schema::{roles, users_roles};

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

        diesel::insert_into(users_roles::table)
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

pub fn get_user_with_roles_by_id(
    id_to_find: i32,
) -> Result<Option<UserWithRolesOutputDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let user = users
        .filter(id.eq(id_to_find))
        .select(User::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    let user = match user {
        Some(user) => user,
        None => return Ok(None),
    };

    let roles: Vec<Role> = roles::table
        .inner_join(users_roles::table)
        .filter(users_roles::user_id.eq(user.id))
        .select(Role::as_select())
        .load(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(Some(UserWithRolesOutputDto {
        id: user.id,
        name: user.name,
        email: user.email,
        roles: roles.into_iter().map(|role| role.into()).collect(),
    }))
}

pub fn set_user_roles(user_id: i32, roles: Vec<RoleEnum>) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Result<(), Box<dyn Error>> = conn.transaction(|tx| {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(user_id))).execute(tx)?;

        diesel::insert_into(users_roles::table)
            .values(
                roles
                    .into_iter()
                    .map(|role| UsersRole {
                        user_id,
                        role_name: role.into(),
                    })
                    .collect::<Vec<UsersRole>>(),
            )
            .execute(tx)?;

        Ok(())
    });

    result.map_err(|_| ServiceError::InternalServerError)?;

    Ok(())
}

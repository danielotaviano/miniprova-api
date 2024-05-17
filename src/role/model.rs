use crate::schema::*;
use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    prelude::Insertable,
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::user::model::User;

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
pub struct Role {
    pub name: RoleEnum,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]

pub enum RoleEnum {
    ADMIN,
    STUDENT,
    TEACHER,
    MONITOR,
}

impl From<RoleEnum> for String {
    fn from(role: RoleEnum) -> Self {
        match role {
            RoleEnum::ADMIN => "ADMIN".to_string(),
            RoleEnum::STUDENT => "STUDENT".to_string(),
            RoleEnum::TEACHER => "TEACHER".to_string(),
            RoleEnum::MONITOR => "MONITOR".to_string(),
        }
    }
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Insertable)]
#[belongs_to(User)]
#[belongs_to(Role, foreign_key = "role_name")]
#[diesel(primary_key(user_id, role_name))]
#[table_name = "users_roles"]

pub struct UsersRole {
    pub user_id: i32,
    pub role_name: String,
}

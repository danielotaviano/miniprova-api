use diesel::{deserialize::Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::*;

use super::enm::RoleEnum;

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = roles)]
pub struct Role {
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<Role> for RoleEnum {
    fn from(value: Role) -> Self {
        match value.name.as_str() {
            "ADMIN" => RoleEnum::ADMIN,
            "STUDENT" => RoleEnum::STUDENT,
            "TEACHER" => RoleEnum::TEACHER,
            "MONITOR" => RoleEnum::MONITOR,
            _ => panic!("Invalid role name"),
        }
    }
}

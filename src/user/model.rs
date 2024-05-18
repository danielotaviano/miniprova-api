use crate::{role::model::Role, schema::*};
use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    Insertable, Selectable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Insertable)]
#[diesel(belongs_to(Role, foreign_key = role_name))]
#[diesel(belongs_to(User))]
#[diesel(primary_key(user_id, role_name))]
#[diesel(table_name = users_roles)]
pub struct UsersRole {
    pub role_name: String,
    pub user_id: i32,
}

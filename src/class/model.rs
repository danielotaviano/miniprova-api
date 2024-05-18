use crate::schema::*;
use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = classes)]
#[diesel(belongs_to(User))]
pub struct Class {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub description: String,
    pub user_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = classes)]
pub struct NewClass<'a> {
    pub name: &'a str,
    pub code: &'a str,
    pub description: &'a str,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = classes)]
pub struct UpdateClass {
    pub name: Option<String>,
    pub description: Option<String>,
}

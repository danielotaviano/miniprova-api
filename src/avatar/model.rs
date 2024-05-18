use crate::schema::*;
use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
pub struct Avatar {
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = avatars)]
pub struct NewAvatar {
    pub user_id: i32,
    pub url: String,
}

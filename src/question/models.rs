use diesel::{prelude::Insertable, Queryable};
use serde::Serialize;

use crate::schema::{answers, questions};

#[derive(Queryable, Serialize)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = questions)]
pub struct NewQuestion<'a> {
    pub question: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct Answer {
    pub id: i32,
    pub answer: String,
    pub is_correct: bool,
    pub question_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = answers)]
pub struct NewAnswer {
    pub answer: String,
    pub is_correct: bool,
    pub question_id: i32,
}

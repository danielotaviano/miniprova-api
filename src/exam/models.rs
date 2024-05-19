use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Exam {
    pub id: i32,
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub class_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = exams)]
pub struct NewExam {
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub class_id: i32,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = exams)]
pub struct UpdateExam {
    pub name: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct StudentAnswer {
    pub id: i32,
    pub user_id: i32,
    pub exam_id: i32,
    pub question_id: i32,
    pub answer_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = student_answers)]
pub struct NewStudentAnswer {
    pub user_id: i32,
    pub exam_id: i32,
    pub question_id: i32,
    pub answer_id: i32,
}

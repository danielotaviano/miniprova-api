use diesel::RunQueryDsl;

use crate::{db::DB_MANAGER, errors::ServiceError, schema::exams};

use super::models::{Exam, NewExam, UpdateExam};
use crate::diesel::*;

pub fn create_exam(new_exam: NewExam) -> Result<Exam, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Exam = diesel::insert_into(exams::table)
        .values(new_exam)
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(result)
}

pub fn get_exam_by_id(exam_id: i32) -> Result<Option<Exam>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let exam = exams::table
        .find(exam_id)
        .first(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(exam)
}

pub fn get_exams_by_class_id(class_id: i32) -> Result<Vec<Exam>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let exams = exams::table
        .filter(exams::class_id.eq(class_id))
        .load(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(exams)
}

pub fn update_exam(exam_id: i32, new_exam: UpdateExam) -> Result<Exam, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Exam = diesel::update(exams::table.find(exam_id))
        .set(new_exam)
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(result)
}

pub fn delete_exam(exam_id: i32) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    diesel::delete(exams::table.find(exam_id))
        .execute(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(())
}

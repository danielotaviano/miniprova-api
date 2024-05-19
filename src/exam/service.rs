use crate::{class, errors::ServiceError};

use super::{
    dto::CreateExamInputDto,
    models::{Exam, NewExam, UpdateExam},
    repository,
};

pub fn create_exam(user_id: i32, new_exam: CreateExamInputDto) -> Result<Exam, ServiceError> {
    let is_teacher = class::service::is_class_teacher(user_id, new_exam.class_id)?;

    if !is_teacher {
        return Err(ServiceError::Forbidden);
    }

    if new_exam.start_date > new_exam.end_date {
        return Err(ServiceError::BadRequest(
            "End date must be after start date".to_string(),
        ));
    }

    if new_exam.start_date < chrono::Utc::now().naive_utc() {
        return Err(ServiceError::BadRequest(
            "Start date must be in the future".to_string(),
        ));
    }

    let exam = repository::create_exam(NewExam {
        name: new_exam.name,
        start_date: new_exam.start_date,
        end_date: new_exam.end_date,
        class_id: new_exam.class_id,
    })?;
    Ok(exam)
}

pub fn get_exam_by_id(exam_id: i32) -> Result<Option<Exam>, ServiceError> {
    let exam = repository::get_exam_by_id(exam_id)?;
    Ok(exam)
}

pub fn get_exams_by_class_id(class_id: i32) -> Result<Vec<Exam>, ServiceError> {
    let exams = repository::get_exams_by_class_id(class_id)?;
    Ok(exams)
}

pub fn update_exam(user_id: i32, exam_id: i32, new_exam: UpdateExam) -> Result<Exam, ServiceError> {
    let existing = repository::get_exam_by_id(exam_id)?;

    if existing.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let existing = existing.unwrap();

    let is_teacher = class::service::is_class_teacher(user_id, existing.class_id)?;

    if !is_teacher {
        return Err(ServiceError::Forbidden);
    }

    if let Some(start_date) = new_exam.start_date {
        if let Some(end_date) = new_exam.end_date {
            if start_date > end_date {
                return Err(ServiceError::BadRequest(
                    "End date must be after start date".to_string(),
                ));
            }
        }
    }

    if let Some(start_date) = new_exam.start_date {
        if start_date < chrono::Utc::now().naive_utc() {
            return Err(ServiceError::BadRequest(
                "Start date must be in the future".to_string(),
            ));
        }
    }

    let exam = repository::update_exam(exam_id, new_exam)?;
    Ok(exam)
}

pub fn delete_exam(user_id: i32, exam_id: i32) -> Result<(), ServiceError> {
    let existing = repository::get_exam_by_id(exam_id)?;

    if existing.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let existing = existing.unwrap();

    let is_teacher = class::service::is_class_teacher(user_id, existing.class_id)?;

    if !is_teacher {
        return Err(ServiceError::Forbidden);
    }

    repository::delete_exam(exam_id)?;
    Ok(())
}

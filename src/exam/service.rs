use crate::{class, errors::ServiceError, question};

use super::{
    dto::{CreateExamInputDto, StudentExamResultDto},
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
    let class = class::service::get_class_by_id(class_id)?;

    if class.is_none() {
        return Err(ServiceError::BadRequest("Class not found".to_string()));
    }

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

pub fn update_questions_in_exam(
    user_id: i32,
    exam_id: i32,
    question_ids: Vec<i32>,
) -> Result<(), ServiceError> {
    let existing = repository::get_exam_by_id(exam_id)?;

    if existing.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let existing = existing.unwrap();

    let is_teacher = class::service::is_class_teacher(user_id, existing.class_id)?;

    if !is_teacher {
        return Err(ServiceError::Forbidden);
    }

    if existing.start_date < chrono::Utc::now().naive_utc() {
        return Err(ServiceError::BadRequest("Exam already started".to_string()));
    }

    let maybe_errors = question_ids
        .iter()
        .map(|question_id| {
            let question = question::service::get_question_by_id(*question_id);
            if question.is_err() {
                return Some(ServiceError::InternalServerError);
            }

            if question.unwrap().is_none() {
                return Some(ServiceError::BadRequest(format!(
                    "Question {} not found",
                    question_id
                )));
            }

            None
        })
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    if !maybe_errors.is_empty() {
        let error = maybe_errors.get(0).unwrap();
        return Err(error.clone());
    }

    repository::update_questions_in_exam(exam_id, question_ids)?;
    Ok(())
}

pub fn get_questions_in_exam_as_student(
    student_id: i32,
    exam_id: i32,
) -> Result<Vec<question::dto::QuestionWithAnswersDto>, ServiceError> {
    let exam = repository::get_exam_by_id(exam_id)?;

    if exam.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let exam = exam.unwrap();

    let is_student = class::service::is_student_enrolled(exam.class_id, student_id)?;

    if !is_student {
        return Err(ServiceError::Forbidden);
    }

    if exam.start_date > chrono::Utc::now().naive_utc() {
        return Err(ServiceError::BadRequest("Exam not started yet".to_string()));
    }

    let mut questions = repository::get_questions_in_exam(exam_id)?;

    if exam.end_date > chrono::Utc::now().naive_utc() {
        for question in questions.iter_mut() {
            for answer in question.answers.iter_mut() {
                answer.is_correct = None;
            }
        }
    }

    Ok(questions)
}

pub fn get_questions_in_exam_as_teacher(
    exam_id: i32,
) -> Result<Vec<question::dto::QuestionWithAnswersDto>, ServiceError> {
    let exam = repository::get_exam_by_id(exam_id)?;

    if exam.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let questions = repository::get_questions_in_exam(exam_id)?;
    Ok(questions)
}

pub fn submit_answer_to_question_in_exam(
    user_id: i32,
    exam_id: i32,
    question_id: i32,
    answer_id: i32,
) -> Result<(), ServiceError> {
    let exam = repository::get_exam_by_id(exam_id)?;

    if exam.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let exam = exam.unwrap();

    let is_enrolled = class::service::is_student_enrolled(exam.class_id, user_id)?;

    if !is_enrolled {
        return Err(ServiceError::Forbidden);
    }

    if exam.start_date > chrono::Utc::now().naive_utc() {
        return Err(ServiceError::BadRequest("Exam not started yet".to_string()));
    }

    if exam.end_date < chrono::Utc::now().naive_utc() {
        return Err(ServiceError::BadRequest("Exam already ended".to_string()));
    }

    let question = question::service::get_question_by_id(question_id)?;

    if question.is_none() {
        return Err(ServiceError::BadRequest("Question not found".to_string()));
    }

    let answers = question::service::list_answers_by_question_id(question_id)?;

    let answer = answers.iter().find(|a| a.id == answer_id);

    if answer.is_none() {
        return Err(ServiceError::BadRequest("Answer not found".to_string()));
    }

    let answer = answer.unwrap();

    if answer.question_id != question_id {
        return Err(ServiceError::BadRequest("Answer not found".to_string()));
    }

    repository::submit_answer_to_question_in_exam(exam_id, question_id, user_id, answer_id)?;
    Ok(())
}

pub fn get_exam_results_as_student(
    user_id: i32,
    exam_id: i32,
) -> Result<StudentExamResultDto, ServiceError> {
    let exam = repository::get_exam_by_id(exam_id)?;

    if exam.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let exam = exam.unwrap();

    if exam.start_date > chrono::Utc::now().naive_utc() {
        return Err(ServiceError::BadRequest("Exam not started yet".to_string()));
    }

    if exam.end_date > chrono::Utc::now().naive_utc() {
        return Err(ServiceError::BadRequest("Exam not ended yet".to_string()));
    }

    let results = repository::get_exam_results_as_student(exam_id, user_id)?;
    Ok(results)
}

pub fn get_exam_results_as_teacher(
    user_id: i32,
    exam_id: i32,
) -> Result<Vec<StudentExamResultDto>, ServiceError> {
    let exam = repository::get_exam_by_id(exam_id)?;

    if exam.is_none() {
        return Err(ServiceError::BadRequest("Exam not found".to_string()));
    }

    let exam = exam.unwrap();

    let is_teacher = class::service::is_class_teacher(user_id, exam.class_id)?;

    if !is_teacher {
        return Err(ServiceError::Forbidden);
    }

    let results = repository::get_exam_results_as_teacher(exam_id)?;
    Ok(results)
}

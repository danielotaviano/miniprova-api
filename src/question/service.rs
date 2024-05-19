use crate::errors::ServiceError;

use super::{
    dto::CreateQuestionInputDto,
    models::{Answer, Question},
    repository,
};

pub fn create_question(new_question: CreateQuestionInputDto) -> Result<(), ServiceError> {
    if new_question.answers.is_empty() {
        return Err(ServiceError::BadRequest(
            "Must have at least 1 answer".to_string(),
        ));
    }

    let correct_answers = new_question.answers.iter().filter(|a| a.is_correct).count();
    if correct_answers != 1 {
        return Err(ServiceError::BadRequest(
            "Must have exactly 1 correct answer".to_string(),
        ));
    }

    let mut answers = new_question.answers.clone();
    answers.sort_by(|a, b| a.answer.cmp(&b.answer));
    for i in 0..answers.len() - 1 {
        if answers[i].answer == answers[i + 1].answer {
            return Err(ServiceError::BadRequest(
                "Cannot have two answers with the same value".to_string(),
            ));
        }
    }

    repository::create_question(new_question)?;

    Ok(())
}

pub fn get_question_by_id(question_id: i32) -> Result<Option<Question>, ServiceError> {
    repository::get_question_by_id(question_id)
}

pub fn delete_question_by_id(question_id: i32) -> Result<(), ServiceError> {
    let existing = repository::get_question_by_id(question_id)?;

    if existing.is_none() {
        return Err(ServiceError::BadRequest("Question not found".to_string()));
    }

    repository::delete_question_by_id(question_id)?;

    Ok(())
}

pub fn list_answers_by_question_id(question_id: i32) -> Result<Vec<Answer>, ServiceError> {
    repository::list_answers_by_question_id(question_id)
}

pub fn list_questions() -> Result<Vec<Question>, ServiceError> {
    repository::list_questions()
}

pub fn update_question(question_id: i32, new_question: CreateQuestionInputDto) -> Result<(), ServiceError> {
    if new_question.answers.is_empty() {
        return Err(ServiceError::BadRequest(
            "Must have at least 1 answer".to_string(),
        ));
    }

    let correct_answers = new_question.answers.iter().filter(|a| a.is_correct).count();
    if correct_answers != 1 {
        return Err(ServiceError::BadRequest(
            "Must have exactly 1 correct answer".to_string(),
        ));
    }

    let mut answers = new_question.answers.clone();
    answers.sort_by(|a, b| a.answer.cmp(&b.answer));
    for i in 0..answers.len() - 1 {
        if answers[i].answer == answers[i + 1].answer {
            return Err(ServiceError::BadRequest(
                "Cannot have two answers with the same value".to_string(),
            ));
        }
    }

    let existing = repository::get_question_by_id(question_id)?;

    if existing.is_none() {
        return Err(ServiceError::BadRequest("Question not found".to_string()));
    }

    repository::update_question(question_id, new_question)?;

    Ok(())
}
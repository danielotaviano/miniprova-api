use actix_web::{web, HttpResponse, Responder};

use crate::errors::ServiceError;

use super::{dto::CreateQuestionInputDto, service};

pub async fn create_question(question: web::Json<CreateQuestionInputDto>) -> impl Responder {
    match question.validate() {
        Err(e) => return HttpResponse::from_error(ServiceError::BadRequest(e)),
        Ok(_) => (),
    }

    match service::create_question(question.into_inner()) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(question) => question,
    };

    HttpResponse::Created().into()
}

pub async fn get_question_by_id(question_id: web::Path<i32>) -> impl Responder {
    let question = match service::get_question_by_id(question_id.into_inner()) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(question) => question,
    };

    match question {
        Some(question) => HttpResponse::Ok().json(question).into(),
        None => HttpResponse::NotFound().finish().into(),
    }
}

pub async fn delete_question_by_id(question_id: web::Path<i32>) -> impl Responder {
    match service::delete_question_by_id(question_id.into_inner()) {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}

pub async fn list_answers_by_question_id(question_id: web::Path<i32>) -> impl Responder {
    let answers = match service::list_answers_by_question_id(question_id.into_inner()) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(answers) => answers,
    };

    HttpResponse::Ok().json(answers).into()
}

pub async fn list_questions() -> impl Responder {
    let questions = match service::list_questions() {
        Err(e) => return HttpResponse::from_error(e),
        Ok(questions) => questions,
    };

    HttpResponse::Ok().json(questions).into()
}

pub async fn update_question_by_id(
    question_id: web::Path<i32>,
    question: web::Json<CreateQuestionInputDto>,
) -> impl Responder {
    match question.validate() {
        Err(e) => return HttpResponse::from_error(ServiceError::BadRequest(e)),
        Ok(_) => (),
    }

    match service::update_question(question_id.into_inner(), question.into_inner()) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(question) => question,
    };

    HttpResponse::Ok().into()
}
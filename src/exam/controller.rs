use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::{auth::models::LoggedUser, errors::ServiceError};

use super::{
    dto::{CreateExamInputDto, StudentAnswerInputDto},
    models::UpdateExam,
    service,
};

pub async fn create_exam(req: HttpRequest, input: web::Json<CreateExamInputDto>) -> impl Responder {
    match input.validate() {
        Err(e) => return HttpResponse::from_error(ServiceError::BadRequest(e)),
        _ => (),
    }

    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    let exam = match service::create_exam(user.id, input.into_inner()) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(exam) => exam,
    };

    HttpResponse::Created().json(exam).into()
}

pub async fn get_exam_by_id(path: web::Path<i32>) -> impl Responder {
    let exam_id = path.into_inner();
    match service::get_exam_by_id(exam_id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(e) => match e {
            None => HttpResponse::NotFound().into(),
            Some(ex) => HttpResponse::Ok().json(ex).into(),
        },
    }
}

pub async fn delete_exam(path: web::Path<i32>, req: HttpRequest) -> impl Responder {
    let exam_id = path.into_inner();

    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    match service::delete_exam(user.id, exam_id) {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}

pub async fn list_exams_by_class_id(path: web::Path<i32>) -> impl Responder {
    let class_id = path.into_inner();
    let exams = match service::get_exams_by_class_id(class_id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(exams) => exams,
    };

    HttpResponse::Ok().json(exams).into()
}

pub async fn update_exam(
    path: web::Path<i32>,
    req: HttpRequest,
    input: web::Json<UpdateExam>,
) -> impl Responder {
    let exam_id = path.into_inner();

    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    let exam = match service::update_exam(user.id, exam_id, input.into_inner()) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(exam) => exam,
    };

    HttpResponse::Ok().json(exam).into()
}

pub async fn update_questions_in_exam(
    path: web::Path<i32>,
    req: HttpRequest,
    input: web::Json<Vec<i32>>,
) -> impl Responder {
    let exam_id = path.into_inner();

    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    match service::update_questions_in_exam(user.id, exam_id, input.into_inner()) {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}

pub async fn get_questions_in_exam_as_student(path: web::Path<i32>) -> impl Responder {
    let exam_id = path.into_inner();

    let questions = match service::get_questions_in_exam_as_student(exam_id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(questions) => questions,
    };

    HttpResponse::Ok().json(questions).into()
}

pub async fn get_questions_in_exam_as_teacher(path: web::Path<i32>) -> impl Responder {
    let exam_id = path.into_inner();

    let questions = match service::get_questions_in_exam_as_teacher(exam_id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(questions) => questions,
    };

    HttpResponse::Ok().json(questions).into()
}

pub async fn submit_answer_to_question_in_exam(
    path: web::Path<(i32, i32)>,
    req: HttpRequest,
    input: web::Json<StudentAnswerInputDto>,
) -> impl Responder {
    let (exam_id, question_id) = path.into_inner();

    match input.validate() {
        Err(e) => return HttpResponse::from_error(ServiceError::BadRequest(e)),
        _ => (),
    }

    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    match service::submit_answer_to_question_in_exam(
        user.id,
        exam_id,
        question_id,
        input.into_inner().answer_id,
    ) {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}

pub async fn get_exam_results_as_student(path: web::Path<i32>, req: HttpRequest) -> impl Responder {
    let exam_id = path.into_inner();

    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    let results = match service::get_exam_results_as_student(user.id, exam_id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(results) => results,
    };

    HttpResponse::Ok().json(results).into()
}

pub async fn get_exam_results_as_teacher(path: web::Path<i32>, req: HttpRequest) -> impl Responder {
    let exam_id = path.into_inner();

    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    let results = match service::get_exam_results_as_teacher(user.id, exam_id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(results) => results,
    };

    HttpResponse::Ok().json(results).into()
}

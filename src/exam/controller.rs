use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::{auth::models::LoggedUser, errors::ServiceError};

use super::{dto::CreateExamInputDto, models::UpdateExam, service};

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
    let exam = match service::get_exam_by_id(exam_id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(exam) => exam,
    };

    HttpResponse::Ok().json(exam).into()
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

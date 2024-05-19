use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::{auth::models::LoggedUser, errors::ServiceError};

use super::{
    dto::{CreateClassInputDto, UpdateClassInputDto},
    service,
};

pub async fn create_class(
    req: HttpRequest,
    class: web::Json<CreateClassInputDto>,
) -> impl Responder {
    match class.validate() {
        Err(e) => return HttpResponse::from_error(ServiceError::BadRequest(e)),
        Ok(_) => (),
    }

    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    let class = match service::create_class(user.id, class.into_inner()) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(class) => class,
    };

    HttpResponse::Created().json(class).into()
}

pub async fn get_class_by_id(path: web::Path<i32>) -> impl Responder {
    let class_id = path.into_inner();
    match service::get_class_by_id(class_id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(class) => match class {
            None => HttpResponse::NotFound().into(),
            Some(class) => HttpResponse::Ok().json(class).into(),
        },
    }
}

pub async fn delete_class(path: web::Path<i32>, req: HttpRequest) -> impl Responder {
    let class_id = path.into_inner();
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    match service::delete_class_by_id(user.id, class_id) {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}

pub async fn update_class(
    path: web::Path<i32>,
    class: web::Json<UpdateClassInputDto>,
    req: HttpRequest,
) -> impl Responder {
    match class.validate() {
        Err(e) => return HttpResponse::from_error(ServiceError::BadRequest(e)),
        Ok(_) => (),
    }

    let class_id = path.into_inner();
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    let class = match service::update_class(user.id, class_id, class.into_inner()) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(class) => class,
    };

    HttpResponse::Ok().json(class).into()
}

pub async fn enroll_student(path: web::Path<i32>, req: HttpRequest) -> impl Responder {
    let class_id = path.into_inner();
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    match service::enroll_student(class_id, user.id) {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().finish(),
    }
}

pub async fn list_classes_that_student_is_enrolled(req: HttpRequest) -> impl Responder {
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    let classes = match service::list_classes_that_student_is_enrolled(user.id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(classes) => classes,
    };

    HttpResponse::Ok().json(classes).into()
}

pub async fn list_classes_that_student_is_not_enrolled(req: HttpRequest) -> impl Responder {
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    let classes = match service::list_classes_that_student_is_not_enrolled(user.id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(classes) => classes,
    };

    HttpResponse::Ok().json(classes).into()
}

pub async fn list_classes_by_teacher(req: HttpRequest) -> impl Responder {
    let ext = req.extensions();
    let user = ext.get::<LoggedUser>().unwrap();

    let classes = match service::list_classes_by_teacher(user.id) {
        Err(e) => return HttpResponse::from_error(e),
        Ok(classes) => classes,
    };

    HttpResponse::Ok().json(classes).into()
}

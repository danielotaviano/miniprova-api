use actix_web::{web, HttpResponse, Responder};

use crate::{
    errors::ServiceError,
    role::enm::RoleEnum,
    user::{dto::CreateUserInputDto, service},
};

pub async fn create_user(new_user: web::Json<CreateUserInputDto>) -> impl Responder {
    if let Err(e) = new_user.validate() {
        return HttpResponse::from_error(ServiceError::BadRequest(e));
    }

    let user = match service::create_user(new_user.into_inner()) {
        Ok(user) => user,
        Err(e) => return HttpResponse::from_error(e),
    };

    HttpResponse::Ok().json(user).into()
}

pub async fn set_user_roles(path: web::Path<i32>, roles: web::Json<Vec<String>>) -> impl Responder {
    let user_id = path.into_inner();
    let roles: Vec<RoleEnum> = roles.into_inner().iter().map(|role| role.into()).collect();

    if roles.contains(&RoleEnum::INVALID) {
        return HttpResponse::from_error(ServiceError::BadRequest("Invalid role".into()));
    }

    match service::set_user_roles(user_id, roles) {
        Ok(_) => HttpResponse::NoContent().into(),
        Err(e) => HttpResponse::from_error(e),
    }
}

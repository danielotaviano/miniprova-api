use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    errors::ServiceError,
    user::{dto::CreateUserInputDto, service},
};

#[post("/user")]
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

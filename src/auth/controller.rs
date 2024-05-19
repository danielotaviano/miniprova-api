use actix_web::{web, HttpResponse, Responder};

use crate::auth::{dto::LoginUserInputDto, service};

pub async fn login(new_user: web::Json<LoginUserInputDto>) -> impl Responder {
    if let Err(e) = new_user.validate() {
        return HttpResponse::from_error(e);
    }

    let output = match service::login(new_user.into_inner()) {
        Ok(output) => output,
        Err(e) => return HttpResponse::from_error(e),
    };

    HttpResponse::Ok().json(output).into()
}

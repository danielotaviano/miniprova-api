use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};

use crate::{
    auth::models::LoggedUser,
    avatar::{self, dto::CreateAvatarInputDto},
    errors::ServiceError,
};

pub async fn update_user_avatar(
    req: HttpRequest,
    new_avatar: web::Json<CreateAvatarInputDto>,
) -> impl Responder {
    let ext = req.extensions();
    let user = match ext.get::<LoggedUser>() {
        Some(user) => user,
        None => return HttpResponse::from_error(ServiceError::Unauthorized),
    };

    if let Err(e) = new_avatar.validate() {
        return HttpResponse::from_error(ServiceError::BadRequest(e));
    }

    let avatar = match avatar::service::create_avatar(user.id, new_avatar.into_inner()) {
        Ok(avatar) => avatar,
        Err(e) => return HttpResponse::from_error(e),
    };

    HttpResponse::Ok().json(avatar).into()
}

pub async fn delete_user_avatar(req: HttpRequest) -> impl Responder {
    let ext = req.extensions();
    let user = match ext.get::<LoggedUser>() {
        Some(user) => user,
        None => return HttpResponse::from_error(ServiceError::Unauthorized),
    };

    match avatar::service::delete_avatar(user.id) {
        Err(e) => HttpResponse::from_error(e),
        Ok(_) => HttpResponse::NoContent().into(),
    }
}

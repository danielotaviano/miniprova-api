use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use crate::{
    auth::{self, models::LoggedUser},
    errors::ServiceError,
    role::enm::RoleEnum,
    user,
};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthMiddleware;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = Authentication<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(Authentication { service }))
    }
}

pub struct Authentication<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for Authentication<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt = match req.headers().get(header::AUTHORIZATION) {
            Some(jwt) => jwt,
            None => {
                return Box::pin(async {
                    let res = req.error_response(ServiceError::Unauthorized);
                    Ok(res)
                });
            }
        }
        .to_str()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap();

        let user_id = match auth::crypto::decode_token(jwt) {
            Ok(user_id) => user_id,
            Err(e) => {
                return Box::pin(async {
                    let res = req.error_response(e);
                    Ok(res)
                });
            }
        };

        let user = match user::service::get_user_with_roles_by_id(user_id) {
            Ok(user) => user,
            Err(e) => {
                return Box::pin(async {
                    let res = req.error_response(e);
                    Ok(res)
                });
            }
        };

        match user {
            Some(user) => {
                let logged_user = LoggedUser {
                    id: user.id,
                    roles: user.roles,
                };
                req.extensions_mut().insert(logged_user);
            }
            None => {
                return Box::pin(async {
                    let res = req.error_response(ServiceError::Unauthorized);
                    Ok(res)
                });
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

pub struct RoleMiddleware(pub Vec<RoleEnum>);

impl<S> Transform<S, ServiceRequest> for RoleMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = RoleAuthentication<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RoleAuthentication {
            service,
            role: self.0.clone(),
        }))
    }
}

pub struct RoleAuthentication<S> {
    service: S,
    role: Vec<RoleEnum>,
}

impl<S> Service<ServiceRequest> for RoleAuthentication<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let has_role;
        {
            let ext = req.extensions();
            let logged_user = ext.get::<LoggedUser>().unwrap();

            has_role = logged_user
                .roles
                .iter()
                .any(|role| self.role.contains(role))
                || logged_user.roles.contains(&RoleEnum::ADMIN);
        }

        let fut: <S as Service<ServiceRequest>>::Future = match has_role {
            true => self.service.call(req),
            false => {
                let res = req.error_response(ServiceError::Forbidden);
                return Box::pin(async { Ok(res) });
            }
        };

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

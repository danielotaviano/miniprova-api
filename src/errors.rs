use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug, Display, Clone)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Forbidden")]
    Forbidden
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json(&ErrorResponse {
                    message: "Internal Server Error. Please try again later".into(),
                })
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(&ErrorResponse {
                    message: message.clone(),
                })
            }
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json(&ErrorResponse {
                message: "Unauthorized".into(),
            }),
            ServiceError::Forbidden => HttpResponse::Forbidden().json(&ErrorResponse {
                message: "Forbidden".into(),
            }),
        }
    }
}

use derive_more::{Display, Error};

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use crate::MyError::*;

#[derive(Debug, Display, Error)]
pub enum MyError {
    #[display(fmt = "no content for requested item")]
    NoContent,

    #[display(fmt = "unauthorized request")]
    UnAuthorized,

    #[display(fmt = "internal error")]
    InternalError,
}

impl actix_web::error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::NoContent => StatusCode::BAD_REQUEST,
            MyError::UnAuthorized => StatusCode::UNAUTHORIZED,
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for MyError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NoContent,
            _ => InternalError
        }
    }
}



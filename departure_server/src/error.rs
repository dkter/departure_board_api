use actix_web::{http::StatusCode, HttpResponse};
use clorinde::tokio_postgres;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error")]
    DbError(#[from] tokio_postgres::Error),
    #[error("Pool error")]
    PoolError(#[from] deadpool::managed::PoolError<tokio_postgres::Error>),
    #[error("InvalidURLError")]
    InvalidURLError(String),
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PoolError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidURLError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
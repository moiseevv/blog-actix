use actix_web::error::BlockingError;
use actix_web::web::HttpResponse;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::{DatabaseError, NotFound};
use std::fmt;

#[derive(Debug)]
pub enum AppError{
    RecordAlredyExists,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    OperationCanceled,
}
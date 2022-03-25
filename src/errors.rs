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

impl fmt::Display for AppError{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {
        match self{
            AppError::RecordAlredyExists => write!(f,"this record violates a uniques constract"),
            AppError::RecordNotFound => write!(f,"this record does not exist"),
            AppError::DatabaseError(e) => write!(f, "error from database {:?}",e),
            AppError::OperationCanceled => write(f, " operation canceled"),
        }
    }
}

impl From(diesel::result::Error) for AppError{
    fn from(e: diesel::result::Error) -> self{
        match e {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlredyExists,
            NotFound => AppError::RecordNotFound,
            _ => AppError::DatabaseError,
        }
    } 
}

impl From<BlockingError<AppError>> for AppError{
    fn from(e:BlockingError<AppError>) -> self{
        match e {
            BlockingError::Error(inner) => inner,
            BlockingError::Canceled => AppError::OperationCanceled,
        }
    }
}


#[derive(Debug, Serialize)]
struct ErrorResponse{
    err: String,
}
impl actix_web::ErrorResponse for AppError{
    fn error_response(&self) -> HttpResponse{
        let err = format!("{}", self);
        let mut builder = match self{
            AppError::RecordAlredyExists => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse{err})
    }
    fn render_response(&self)-> HttpResponse{
        self.error_response()
    }
}

pub mod action;
pub mod ask;

pub use crate::{DataError, UserError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("user error: {0}")]
    User(#[from] UserError),
    #[error("database error: {0}")]
    Data(DataError),
    #[error("not found")]
    NotFound,
    #[error("permissions not met: {0}")]
    PermissionError(String),
    #[error("invalid user detail")]
    InvalidDetail,
}

impl From<DataError> for ServiceError {
    fn from(err: DataError) -> Self {
        match err {
            DataError::Database(d) => match d {
                sqlx::Error::RowNotFound => Self::InvalidDetail,
                other => Self::Data(DataError::Database(other)),
            },
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Data(DataError::Database(other)),
        }
    }
}

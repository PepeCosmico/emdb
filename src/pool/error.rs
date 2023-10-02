use thiserror::Error as ThisError;

use crate::database;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Database error")]
    DbError(database::Error),
}

impl From<database::Error> for Error {
    fn from(value: database::Error) -> Self {
        Self::DbError(value)
    }
}

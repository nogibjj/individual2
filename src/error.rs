use std::fmt;
use std::io;
use rusqlite::Error as RusqliteError;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(RusqliteError),
    IoError(io::Error),
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::IoError(e) => write!(f, "I/O error: {}", e),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl From<RusqliteError> for AppError {
    fn from(error: RusqliteError) -> Self {
        AppError::DatabaseError(error)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

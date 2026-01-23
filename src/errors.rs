use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DbError(#[from] rusqlite::Error),

    #[error("Subject '{0}' not found")]
    SubjectNotFound(String),

    #[error("Subject '{0}' already exists")]
    SubjectAlreadyExists(String),

    #[error("Task '{0}' not found")]
    TaskNotFound(String),

    #[error("Failed to insert task '{0}': {1}")]
    TaskInsertErrorWithDb(String, rusqlite::Error),

    #[error("File '{0}' not found")]
    FileNotFound(String),

    #[error("File '{0}' already exists: {1}")]
    FileAlreadyExists(String, rusqlite::Error),

    #[error("Failed to insert file '{0}': {1}")]
    FileInsertError(String, rusqlite::Error),
}

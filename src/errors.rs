use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("database error: {0}")]
    DbError(#[from] rusqlite::Error),

    #[error("subject '{0}' not found")]
    SubjectNotFound(String),

    #[error("subject '{0}' already exists")]
    SubjectAlreadyExists(String),

    #[error("task '{0}' not found")]
    TaskNotFound(String),

    #[error("failed to insert task '{0}': {1}")]
    TaskInsertErrorWithDb(String, rusqlite::Error),

    #[error("file '{0}' not found")]
    FileNotFound(String),

    #[error("file '{0}' already exists: {1}")]
    FileAlreadyExists(String, rusqlite::Error),

    #[error("failed to insert file '{0}': {1}")]
    FileInsertError(String, rusqlite::Error),
}

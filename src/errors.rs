use rusqlite::Error as RusqliteError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("failed to insert file `{0}` into database")]
    FileInsertError(String, #[source] RusqliteError),

    #[error("failed to insert task '{0}'")]
    TaskInsertErrorWithDb(String, #[source] RusqliteError),

    #[error("subject `{0}` not found in workspace")]
    SubjectNotFound(String),

    #[error("file `{0}` not found in workspace")]
    FileNotFound(String),

    #[error("task `{0}` not found")]
    TaskNotFound(String),

    #[error("file `{0}` already exists")]
    FileAlreadyExists(String, #[source] RusqliteError),

    #[error("subject `{0}` already exists in workspace")]
    SubjectAlreadyExists(String),

    #[error("database error")]
    DbError(#[from] rusqlite::Error),

    #[error("failed to prepare subjects query")]
    SubjectsPrepareError,

    #[error("failed to query subjects")]
    SubjectsQueryError,

    #[error("failed to get subject row")]
    SubjectRowError,

    #[error("failed to prepare files query for subject '{0}'")]
    FilesPrepareError(String),

    #[error("failed to query files for subject '{0}'")]
    FilesQueryError(String),

    #[error("failed to get file name")]
    FileNameError,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("failed to insert file `{0}` into database")]
    FileInsertError(String),

    #[error("failed to insert task `{0}` into database")]
    TaskInsertError(String),

    #[error("subject `{0}` not found in workspace")]
    SubjectNotFound(String),

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

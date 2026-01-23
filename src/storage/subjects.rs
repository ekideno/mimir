use super::Storage;
use crate::errors::StorageError;
use crate::models::Subject;
use rusqlite::params;

impl Storage {
    /// Add a new subject
    pub fn add_subject(&self, name: &str) -> Result<(), StorageError> {
        self.conn
            .execute("INSERT INTO subjects (name) VALUES (?1)", [name])
            .map_err(|e| match e {
                rusqlite::Error::SqliteFailure(err, _)
                    if err.code == rusqlite::ffi::ErrorCode::ConstraintViolation =>
                {
                    StorageError::SubjectAlreadyExists(name.to_string())
                }
                other => StorageError::DbError(other),
            })?;

        Ok(())
    }

    /// Delete a subject by name
    pub fn delete_subject(&self, subject_name: &str) -> Result<(), StorageError> {
        let affected = self
            .conn
            .execute("DELETE FROM subjects WHERE name = ?1", [subject_name])?;

        if affected == 0 {
            return Err(StorageError::SubjectNotFound(subject_name.to_string()));
        }

        Ok(())
    }

    /// Rename a subject
    pub fn rename_subject(
        &self,
        subject_name: &str,
        new_subject_name: &str,
    ) -> Result<(), StorageError> {
        let affected = self
            .conn
            .execute(
                "UPDATE subjects SET name = ?1 WHERE name = ?2",
                params![new_subject_name, subject_name],
            )
            .map_err(|e| match e {
                rusqlite::Error::SqliteFailure(err, _)
                    if err.code == rusqlite::ffi::ErrorCode::ConstraintViolation =>
                {
                    StorageError::SubjectAlreadyExists(new_subject_name.to_string())
                }
                other => StorageError::DbError(other),
            })?;

        if affected == 0 {
            return Err(StorageError::SubjectNotFound(subject_name.to_string()));
        }

        Ok(())
    }

    /// Get subject ID by name (case-sensitive as per COLLATE NOCASE in schema)
    pub fn get_subject_id_by_name(&self, subject_name: &str) -> Result<i64, StorageError> {
        self.get_subject_id(subject_name)
    }

    /// Get subject name by ID
    pub fn get_subject_name_by_id(&self, id: i64) -> Result<String, StorageError> {
        self.conn
            .query_row("SELECT name FROM subjects WHERE id = ?1", [id], |row| {
                row.get(0)
            })
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    StorageError::SubjectNotFound(id.to_string())
                }
                other => StorageError::DbError(other),
            })
    }

    /// Get all subjects (sorted alphabetically)
    pub fn get_all_subjects(&self) -> Result<Vec<String>, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM subjects ORDER BY name ASC")?;

        let subjects = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;

        Ok(subjects)
    }

    /// Get all subjects with their files
    pub fn get_all_subjects_with_files(&self) -> Result<Vec<Subject>, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM subjects ORDER BY name ASC")?;

        let subjects = stmt
            .query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = Vec::new();

        for (id, name) in subjects {
            let files = self.get_files_by_subject_id(id)?;
            result.push(Subject { name, files });
        }

        Ok(result)
    }
    pub fn get_subject_name_by_name_ci(&self, name: &str) -> Result<String, StorageError> {
        self.conn
            .query_row("SELECT name FROM subjects WHERE name = ?1", [name], |row| {
                row.get(0)
            })
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    StorageError::SubjectNotFound(name.to_string())
                }
                other => StorageError::DbError(other),
            })
    }
}

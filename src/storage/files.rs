use super::Storage;
use crate::errors::StorageError;
use rusqlite::params;

impl Storage {
    /// Add a file to a subject
    pub fn add_file(&self, subject_id: i64, file_name: &str) -> Result<(), StorageError> {
        self.conn
            .execute(
                "INSERT INTO files (subject_id, name) VALUES (?1, ?2)",
                params![subject_id, file_name],
            )
            .map_err(|e| match e {
                rusqlite::Error::SqliteFailure(ref err, _)
                    if err.code == rusqlite::ErrorCode::ConstraintViolation =>
                {
                    StorageError::FileAlreadyExists(file_name.to_string(), e)
                }
                other => StorageError::FileInsertError(file_name.to_string(), other),
            })?;

        Ok(())
    }

    /// Delete a file by name
    pub fn delete_file(&self, file_name: &str) -> Result<(), StorageError> {
        let affected = self
            .conn
            .execute("DELETE FROM files WHERE name = ?1", [file_name])?;

        if affected == 0 {
            return Err(StorageError::FileNotFound(file_name.to_string()));
        }

        Ok(())
    }

    /// Rename a file
    pub fn rename_file(
        &self,
        old_file_name: &str,
        new_file_name: &str,
    ) -> Result<(), StorageError> {
        let affected = self.conn.execute(
            "UPDATE files SET name = ?1 WHERE name = ?2",
            [new_file_name, old_file_name],
        )?;

        if affected == 0 {
            return Err(StorageError::FileNotFound(old_file_name.to_string()));
        }

        Ok(())
    }

    /// Get subject ID by filename
    pub fn get_subject_id_by_filename(&self, file_name: &str) -> Result<i64, StorageError> {
        self.conn
            .query_row(
                "SELECT subject_id FROM files WHERE name = ?1",
                [file_name],
                |row| row.get(0),
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    StorageError::FileNotFound(file_name.to_string())
                }
                other => StorageError::DbError(other),
            })
    }

    /// Get all files for a subject by subject ID
    pub fn get_files_by_subject_id(&self, subject_id: i64) -> Result<Vec<String>, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM files WHERE subject_id = ?1 ORDER BY name ASC")?;

        let files = stmt
            .query_map([subject_id], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;

        Ok(files)
    }
}

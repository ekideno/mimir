use crate::errors::StorageError;
use crate::models::Subject;
use anyhow::{Context, Error, Result};
use rusqlite::{Connection, params};

pub struct Storage {
    conn: Connection,
}
pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS subjects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subject_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            FOREIGN KEY(subject_id) REFERENCES subjects(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subject_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            done INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY(subject_id) REFERENCES subjects(id) ON DELETE CASCADE
        );
    "#,
    )
    .context("Failed to create tables")?;

    Ok(())
}

impl Storage {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
    pub fn add_file(&self, subject_name: &str, file_name: &str) -> Result<(), StorageError> {
        let subject_id: i64 = self
            .conn
            .query_row(
                "SELECT id FROM subjects WHERE name = ?1",
                [subject_name],
                |row| row.get(0),
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    StorageError::SubjectNotFound(subject_name.to_string())
                }
                other => StorageError::DbError(other),
            })?;

        self.conn
            .execute(
                "INSERT INTO files (subject_id, name) VALUES (?1, ?2)",
                params![subject_id, file_name],
            )
            .map_err(|_| StorageError::FileInsertError(file_name.to_string()))?;

        Ok(())
    }

    pub fn add_task(&self, subject_name: &str, task_title: &str) -> Result<(), StorageError> {
        let subject_id: i64 = self
            .conn
            .query_row(
                "SELECT id FROM subjects WHERE name = ?1",
                [subject_name],
                |row| row.get(0),
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    StorageError::SubjectNotFound(subject_name.to_string())
                }
                other => StorageError::DbError(other),
            })?;

        self.conn
            .execute(
                "INSERT INTO tasks (subject_id, title) VALUES (?1, ?2)",
                params![subject_id, task_title],
            )
            .map_err(|e| StorageError::TaskInsertErrorWithDb(task_title.to_string(), e))?;

        Ok(())
    }

    pub fn add_subject(&self, name: &str) -> Result<(), StorageError> {
        match self
            .conn
            .execute("INSERT INTO subjects (name) VALUES (?1)", [name])
        {
            Ok(_) => Ok(()),
            Err(rusqlite::Error::SqliteFailure(err, _))
                if err.code == rusqlite::ffi::ErrorCode::ConstraintViolation =>
            {
                Err(StorageError::SubjectAlreadyExists(name.to_string()))
            }
            Err(e) => Err(StorageError::DbError(e)),
        }
    }
    pub fn delete_subject(&self, subject_name: &str) -> Result<(), StorageError> {
        let affected = self
            .conn
            .execute("DELETE FROM subjects WHERE name = ?1", [subject_name])
            .map_err(StorageError::DbError)?;
        if affected == 0 {
            return Err(StorageError::SubjectNotFound(subject_name.to_string()));
        }

        Ok(())
    }

    pub fn rename_subject(
        &self,
        subject_name: &str,
        new_subject_name: &str,
    ) -> Result<(), StorageError> {
        match self.conn.execute(
            "UPDATE subjects SET name = ?1 WHERE name = ?2",
            params![new_subject_name, subject_name],
        ) {
            Ok(0) => Err(StorageError::SubjectNotFound(subject_name.to_string())),
            Ok(_) => Ok(()),
            Err(rusqlite::Error::SqliteFailure(err, _))
                if err.code == rusqlite::ffi::ErrorCode::ConstraintViolation =>
            {
                Err(StorageError::SubjectAlreadyExists(
                    new_subject_name.to_string(),
                ))
            }
            Err(e) => Err(StorageError::DbError(e)),
        }
    }

    pub fn get_all_subjects_with_files(&self) -> Result<Vec<Subject>, StorageError> {
        let mut subjects_stmt = self
            .conn
            .prepare("SELECT id, name FROM subjects")
            .map_err(|_| StorageError::SubjectsPrepareError)?;

        let subjects_iter = subjects_stmt
            .query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|_| StorageError::SubjectsQueryError)?;

        let mut result = Vec::new();

        for subj in subjects_iter {
            let (id, name) = subj.map_err(|_| StorageError::SubjectRowError)?;

            let mut files_stmt = self
                .conn
                .prepare("SELECT name FROM files WHERE subject_id = ?1")
                .map_err(|_| StorageError::FilesPrepareError(name.clone()))?;

            let files_iter = files_stmt
                .query_map([id], |row| row.get::<_, String>(0))
                .map_err(|_| StorageError::FilesQueryError(name.clone()))?;

            let mut files = Vec::new();
            for f in files_iter {
                let file_name: String = f.map_err(|_| StorageError::FileNameError)?;
                files.push(file_name);
            }

            result.push(Subject { name, files });
        }

        Ok(result)
    }

    pub fn get_subject(&self, subject_name: &str) -> Result<Subject> {
        let id: i64 = self
            .conn
            .query_row(
                "SELECT id FROM subjects WHERE name = ?1",
                [subject_name],
                |row| Ok(row.get(0)?),
            )
            .context(format!("Subject '{}' not found", subject_name))?;

        let mut files_stmt = self
            .conn
            .prepare("SELECT name FROM files WHERE subject_id = ?1")?;
        let files_iter = files_stmt.query_map([id], |row| row.get::<_, String>(0))?;

        let mut files = Vec::new();
        for f in files_iter {
            files.push(f.context("Failed to get file name")?);
        }

        Ok(Subject {
            name: subject_name.to_string(),
            files,
        })
    }
    pub fn get_all_subjects(&self) -> Result<Vec<Subject>, Error> {
        let mut stmt = self.conn.prepare("SELECT name FROM subjects")?;
        let rows = stmt.query_map([], |row| Ok(Subject::new(row.get(0)?)))?;

        let mut subjects = Vec::new();
        for subj in rows {
            subjects.push(subj?);
        }

        Ok(subjects)
    }
}

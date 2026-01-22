use crate::models::Subject;
use crate::{errors::StorageError, models::Task};
use anyhow::{Context, Result};
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
            .map_err(|e| StorageError::FileInsertError(file_name.to_string(), e))?;

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
    pub fn delete_task(&self, subject_id: i64, task_title: &str) -> Result<(), StorageError> {
        let affected = self.conn.execute(
            "DELETE FROM tasks WHERE subject_id = ?1 AND title = ?2",
            params![subject_id, task_title],
        )?;

        if affected == 0 {
            return Err(StorageError::TaskNotFound(task_title.to_string()));
        }

        Ok(())
    }
    pub fn get_subject_id_by_name(&self, subject_name: &str) -> Result<i64, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM subjects WHERE name = ?1")
            .map_err(StorageError::DbError)?;

        let mut rows = stmt.query([subject_name]).map_err(StorageError::DbError)?;

        if let Some(row) = rows.next().map_err(StorageError::DbError)? {
            let id: i64 = row.get(0).map_err(StorageError::DbError)?;
            Ok(id)
        } else {
            Err(StorageError::SubjectNotFound(subject_name.to_string()))
        }
    }

    pub fn rename_task(
        &self,
        subject_id: i64,
        old_task_title: &str,
        new_task_title: &str,
    ) -> Result<(), StorageError> {
        let affected = self.conn.execute(
            "UPDATE tasks SET title = ?1 WHERE subject_id = ?2 AND title = ?3",
            params![new_task_title, subject_id, old_task_title],
        )?;

        if affected == 0 {
            return Err(StorageError::TaskNotFound(old_task_title.to_string()));
        }

        Ok(())
    }

    pub fn get_subject_id_by_filename(&self, file_name: &str) -> Result<i64, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT subject_id FROM files WHERE name = ?1")
            .map_err(StorageError::DbError)?;

        let mut rows = stmt.query([file_name]).map_err(StorageError::DbError)?;

        if let Some(row) = rows.next().map_err(StorageError::DbError)? {
            let id: i64 = row.get(0).map_err(StorageError::DbError)?;
            Ok(id)
        } else {
            Err(StorageError::FileNotFound(file_name.to_string()))
        }
    }

    pub fn get_subject_id_by_name_ci(&self, subject_name: &str) -> Result<i64, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM subjects WHERE LOWER(name) = LOWER(?1)")
            .map_err(StorageError::DbError)?;

        let mut rows = stmt.query([subject_name]).map_err(StorageError::DbError)?;

        if let Some(row) = rows.next().map_err(StorageError::DbError)? {
            let id: i64 = row.get(0).map_err(StorageError::DbError)?;
            Ok(id)
        } else {
            Err(StorageError::SubjectNotFound(subject_name.to_string()))
        }
    }

    pub fn get_subject_name_by_id(&self, id: i64) -> Result<String, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM subjects WHERE id = ?1")
            .map_err(StorageError::DbError)?;

        let mut rows = stmt.query([id]).map_err(StorageError::DbError)?;

        if let Some(row) = rows.next().map_err(StorageError::DbError)? {
            let name: String = row.get(0).map_err(StorageError::DbError)?;
            Ok(name)
        } else {
            Err(StorageError::SubjectNotFound(id.to_string()))
        }
    }
    pub fn rename_file(
        &self,
        old_file_name: &str,
        new_file_name: &str,
    ) -> Result<(), StorageError> {
        let affected = self
            .conn
            .execute(
                "UPDATE files SET name = ?1 WHERE name = ?2",
                [new_file_name, old_file_name],
            )
            .map_err(StorageError::DbError)?;

        if affected == 0 {
            return Err(StorageError::FileNotFound(old_file_name.to_string()));
        }

        Ok(())
    }

    pub fn delete_file(&self, file_name: &str) -> Result<(), StorageError> {
        let affected = self
            .conn
            .execute("DELETE FROM files WHERE name = ?1", [file_name])
            .map_err(StorageError::DbError)?;
        if affected == 0 {
            return Err(StorageError::FileNotFound(file_name.to_string()));
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

    pub fn get_all_subjects(&self) -> Result<Vec<String>, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM subjects ORDER BY name ASC")
            .map_err(StorageError::DbError)?;

        let mut rows = stmt.query([]).map_err(StorageError::DbError)?;

        let mut subjects = Vec::new();
        while let Some(row) = rows.next().map_err(StorageError::DbError)? {
            let name: String = row.get(0).map_err(StorageError::DbError)?;
            subjects.push(name);
        }

        Ok(subjects)
    }

    pub fn get_task_progress(&self, subject_name: &str) -> Result<(usize, usize), StorageError> {
        let subject_id = self.get_subject_id_by_name(subject_name)?;

        let (total, completed): (i64, i64) = self
            .conn
            .query_row(
                "SELECT COUNT(*), COALESCE(SUM(done), 0) FROM tasks WHERE subject_id = ?1",
                [subject_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .map_err(StorageError::DbError)?;

        Ok((total as usize, completed as usize))
    }
    pub fn get_tasks_by_subject(&self, subject_name: &str) -> Result<Vec<Task>, StorageError> {
        let subject_id = self.get_subject_id_by_name(subject_name)?;

        let mut stmt = self
            .conn
            .prepare("SELECT title, done FROM tasks WHERE subject_id = ?1 ORDER BY rowid ASC")
            .map_err(StorageError::DbError)?;

        let mut rows = stmt.query([subject_id]).map_err(StorageError::DbError)?;
        let mut tasks = Vec::new();

        while let Some(row) = rows.next().map_err(StorageError::DbError)? {
            tasks.push(Task {
                title: row.get(0).map_err(StorageError::DbError)?,
                done: row.get::<_, i64>(1).map_err(StorageError::DbError)? != 0,
            });
        }

        Ok(tasks)
    }
}

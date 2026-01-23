use super::Storage;
use crate::errors::StorageError;
use crate::models::Task;
use rusqlite::params;

impl Storage {
    /// Add a task to a subject
    pub fn add_task(&self, subject_name: &str, task_title: &str) -> Result<(), StorageError> {
        let subject_id = self.get_subject_id(subject_name)?;

        self.conn
            .execute(
                "INSERT INTO tasks (subject_id, title) VALUES (?1, ?2)",
                params![subject_id, task_title],
            )
            .map_err(|e| StorageError::TaskInsertErrorWithDb(task_title.to_string(), e))?;

        Ok(())
    }

    /// Delete a task
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

    /// Rename a task
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

    /// Set task completion status
    pub fn set_task_done(
        &self,
        subject_id: i64,
        task_title: &str,
        done: bool,
    ) -> Result<(), StorageError> {
        let affected = self.conn.execute(
            "UPDATE tasks SET done = ?1 WHERE subject_id = ?2 AND title = ?3",
            params![if done { 1 } else { 0 }, subject_id, task_title],
        )?;

        if affected == 0 {
            return Err(StorageError::TaskNotFound(task_title.to_string()));
        }

        Ok(())
    }

    /// Get all tasks for a subject
    pub fn get_tasks_by_subject(&self, subject_name: &str) -> Result<Vec<Task>, StorageError> {
        let subject_id = self.get_subject_id(subject_name)?;

        let mut stmt = self
            .conn
            .prepare("SELECT title, done FROM tasks WHERE subject_id = ?1 ORDER BY rowid ASC")?;

        let tasks = stmt
            .query_map([subject_id], |row| {
                Ok(Task {
                    title: row.get(0)?,
                    done: row.get::<_, i64>(1)? != 0,
                })
            })?
            .collect::<Result<Vec<Task>, _>>()?;

        Ok(tasks)
    }

    /// Get task progress (total, completed) for a subject
    pub fn get_task_progress(&self, subject_name: &str) -> Result<(usize, usize), StorageError> {
        let subject_id = self.get_subject_id(subject_name)?;

        let (total, completed): (i64, i64) = self.conn.query_row(
            "SELECT COUNT(*), COALESCE(SUM(done), 0) FROM tasks WHERE subject_id = ?1",
            [subject_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        Ok((total as usize, completed as usize))
    }
}

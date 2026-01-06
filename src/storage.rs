use crate::models::Subject;
use anyhow::{Context, Result};
use rusqlite::Connection;

pub struct Storage {
    conn: Connection,
}
pub fn init_db(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS subjects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            task_count INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subject_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            FOREIGN KEY(subject_id) REFERENCES subjects(id) ON DELETE CASCADE
        );
    "#,
    )
    .context("Failed to create tables")?;

    Ok(())
}

impl Storage {
    // pub fn find_subject(&self, name: &str) -> Option<Subject> {
    //     // SQL
    // }
    //
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
    pub fn test_connection(&self) -> rusqlite::Result<()> {
        let _: i32 = self.conn.query_row("SELECT 1", [], |row| row.get(0))?;
        Ok(())
    }
    pub fn add_file(&self, subject_name: &str, file_name: &str) -> anyhow::Result<()> {
        let subject_id: i64 = self
            .conn
            .query_row(
                "SELECT id FROM subjects WHERE name = ?1",
                [subject_name],
                |row| row.get(0),
            )
            .context("Subject not found")?;

        self.conn
            .execute(
                "INSERT INTO files (subject_id, name) VALUES (?1, ?2)",
                rusqlite::params![subject_id, file_name],
            )
            .context("Failed to insert file")?;

        println!("✓ File '{}' added to subject '{}'", file_name, subject_name);
        Ok(())
    }

    pub fn add_subject_names(&self, subject: Subject) -> Option<Subject> {
        match self.conn.execute(
            "INSERT INTO subjects (name, task_count) VALUES (?1, ?2)",
            [&subject.name, &subject.task_count.to_string()],
        ) {
            Ok(_) => Some(subject),
            Err(e) => {
                eprintln!("Ошибка при добавлении: {}", e);
                None
            }
        }
    }

    pub fn get_all_subjects_with_files(&self) -> anyhow::Result<Vec<Subject>> {
        let mut subjects_stmt = self
            .conn
            .prepare("SELECT id, name, task_count FROM subjects")?;
        let subjects_iter = subjects_stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, u32>(2)?,
            ))
        })?;

        let mut result = Vec::new();

        for subj in subjects_iter {
            let (id, name, task_count) = subj?;

            let mut files_stmt = self
                .conn
                .prepare("SELECT name FROM files WHERE subject_id = ?1")?;
            let files_iter = files_stmt.query_map([id], |row| row.get::<_, String>(0))?;

            let mut files = Vec::new();
            for f in files_iter {
                let file_name: String = f.context("Failed to get file name")?;
                files.push(file_name);
            }

            result.push(Subject {
                name,
                task_count,
                files,
            });
        }

        Ok(result)
    }
    pub fn get_subject(&self, subject_name: &str) -> Result<Subject> {
        let (id, task_count): (i64, u32) = self
            .conn
            .query_row(
                "SELECT id, task_count FROM subjects WHERE name = ?1",
                [subject_name],
                |row| Ok((row.get(0)?, row.get(1)?)),
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
            task_count,
            files,
        })
    }
    pub fn get_all_subjects(&self) -> Result<Vec<Subject>> {
        let mut stmt = self.conn.prepare("SELECT name, task_count FROM subjects")?;
        let rows = stmt.query_map([], |row| Ok(Subject::new(row.get(0)?, row.get(1)?)))?;

        let mut subjects = Vec::new();
        for subj in rows {
            subjects.push(subj?);
        }

        Ok(subjects)
    }
}

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
    // pub fn find_subject(&self, name: &str) -> Option<Subject> {
    //     // SQL
    // }
    //
    pub fn new(conn: Connection) -> Self {
        Self { conn }
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
    pub fn add_task(&self, subject_name: &str, task_title: &str) -> anyhow::Result<()> {
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
                "INSERT INTO tasks (subject_id, title) VALUES (?1, ?2)",
                rusqlite::params![subject_id, task_title],
            )
            .context("Failed to insert file")?;

        println!(
            "✓ Task '{}' added to subject '{}'",
            task_title, subject_name
        );
        Ok(())
    }

    pub fn add_subject_names(&self, subject: Subject) -> Option<Subject> {
        match self
            .conn
            .execute("INSERT INTO subjects (name) VALUES (?1)", [&subject.name])
        {
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
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut result = Vec::new();

        for subj in subjects_iter {
            let (id, name) = subj?;

            let mut files_stmt = self
                .conn
                .prepare("SELECT name FROM files WHERE subject_id = ?1")?;
            let files_iter = files_stmt.query_map([id], |row| row.get::<_, String>(0))?;

            let mut files = Vec::new();
            for f in files_iter {
                let file_name: String = f.context("Failed to get file name")?;
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
    pub fn get_all_subjects(&self) -> Result<Vec<Subject>> {
        let mut stmt = self.conn.prepare("SELECT name FROM subjects")?;
        let rows = stmt.query_map([], |row| Ok(Subject::new(row.get(0)?)))?;

        let mut subjects = Vec::new();
        for subj in rows {
            subjects.push(subj?);
        }

        Ok(subjects)
    }
}

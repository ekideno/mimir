use anyhow::{Context, Result};
use rusqlite::Connection;

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS subjects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE COLLATE NOCASE
        );

        CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subject_id INTEGER NOT NULL,
            name TEXT NOT NULL UNIQUE,
            FOREIGN KEY(subject_id) REFERENCES subjects(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subject_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            done INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY(subject_id) REFERENCES subjects(id) ON DELETE CASCADE
        );


        CREATE INDEX IF NOT EXISTS idx_files_subject_id ON files(subject_id);
        CREATE INDEX IF NOT EXISTS idx_tasks_subject_id ON tasks(subject_id);
        CREATE INDEX IF NOT EXISTS idx_subjects_name_lower ON subjects(LOWER(name));
        "#,
    )
    .context("Failed to create tables")?;

    Ok(())
}

mod files;
mod schema;
mod subjects;
mod tasks;

use crate::errors::StorageError;
use anyhow::Result;
use rusqlite::Connection;

pub use schema::init_db;

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    // Helper method to get subject_id by name
    fn get_subject_id(&self, name: &str) -> Result<i64, StorageError> {
        self.conn
            .query_row("SELECT id FROM subjects WHERE name = ?1", [name], |row| {
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

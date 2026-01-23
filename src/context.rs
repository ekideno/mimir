use crate::config::Config;
use crate::storage::{Storage, init_db};
use anyhow::Result;
use rusqlite::Connection;

pub struct AppContext {
    pub config: Config,
    pub storage: Storage,
}

impl AppContext {
    pub fn init() -> Result<Self> {
        let config = Config::load()?;

        let conn = Connection::open(&config.data_path)?;
        init_db(&conn)?;

        let storage = Storage::new(conn);

        Ok(Self { config, storage })
    }
}

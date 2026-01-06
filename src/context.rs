use crate::{
    config::Config,
    storage::{self, Storage},
};
use rusqlite::Connection;

pub struct AppContext {
    pub config: Config,
    pub storage: Storage,
}

impl AppContext {
    pub fn init() -> anyhow::Result<Self> {
        let config = Config::load()?;

        let conn = Connection::open(&config.data_path)?;
        storage::init_db(&conn)?;

        let storage = Storage::new(conn);

        Ok(Self { config, storage })
    }
}

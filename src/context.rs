use crate::config::Config;

#[derive(Debug)]
pub struct AppContext {
    pub config: Config,
    // pub db: Connection,
}

impl AppContext {
    pub fn init() -> anyhow::Result<Self> {
        let config = Config::load()?;
        // let db = Connection::open(&config.db_path)?;
        Ok(Self { config })
    }
}

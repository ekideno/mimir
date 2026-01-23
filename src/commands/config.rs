use crate::context::AppContext;
use anyhow::{Result, anyhow};
use colored::Colorize;
use std::path::PathBuf;

use crate::utils::open_path;

pub fn handle(_ctx: &AppContext) -> Result<()> {
    let home = std::env::var("HOME").map_err(|_| anyhow!("HOME is not set"))?;

    let config_path = PathBuf::from(home)
        .join(".config")
        .join("mimir")
        .join("config");

    if !config_path.exists() {
        return Err(anyhow!("config file {:?} not found", config_path));
    }

    println!(
        "{} config: {}",
        "opening".green().bold(),
        config_path.display()
    );

    open_path(&config_path)?;
    Ok(())
}

use anyhow::{Result, anyhow};
use colored::Colorize;

use crate::AppContext;
use crate::utils::open_path;

pub fn handle(ctx: &AppContext) -> Result<()> {
    let workspace_path = &ctx.config.subjects_path;

    if !workspace_path.is_dir() {
        return Err(anyhow!("workspace folder {:?} not found", workspace_path));
    }

    println!(
        "{} workspace: {}",
        "opening".green().bold(),
        workspace_path.display()
    );

    open_path(workspace_path)?;
    Ok(())
}

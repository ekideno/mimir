use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub fn open_path(path: &Path) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args([
                "/C",
                "start",
                "",
                path.to_str().context("Invalid UTF-8 path")?,
            ])
            .status()
            .context("Failed to execute 'start' command")?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .status()
            .context("Failed to execute 'open' command")?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .status()
            .context("Failed to execute 'xdg-open' command")?;
    }

    Ok(())
}

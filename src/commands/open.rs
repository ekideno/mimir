use crate::context::AppContext;
use anyhow::{Context, Result, anyhow};
use clap::Args;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

#[derive(Args)]
pub struct OpenArgs {
    /// Name of the file or subject to open
    pub name: String,
}

pub fn handle(ctx: &AppContext, args: &OpenArgs) -> Result<()> {
    let subjects_root = &ctx.config.subjects_path;

    let subject_dir = subjects_root.join(&args.name);
    if subject_dir.is_dir() {
        open_path(&subject_dir)?;
    }

    if let Some(entry) = WalkDir::new(subjects_root)
        .into_iter()
        .filter_map(Result::ok)
        .find(|e| {
            e.file_type().is_file() && e.file_name().to_str().map_or(false, |s| s == args.name)
        })
    {
        open_path(entry.path())?;
    }

    Err(anyhow!("File or subject '{}' not found", args.name))
}

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

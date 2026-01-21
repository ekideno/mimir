use crate::context::AppContext;
use anyhow::{Context, Result, anyhow};
use clap::Args;
use colored::*;
use std::path::Path;
use std::process::Command;

#[derive(Args)]
pub struct OpenArgs {
    /// Name of the file or subject to open
    pub target: String,
}

pub fn handle(ctx: &AppContext, args: &OpenArgs) -> Result<()> {
    let subjects_root = &ctx.config.subjects_path;

    if let Ok(subject_id) = ctx.storage.get_subject_id_by_filename(&args.target) {
        let subject_name = ctx.storage.get_subject_name_by_id(subject_id)?;
        let file_path = subjects_root.join(&subject_name).join(&args.target);

        if !file_path.is_file() {
            return Err(anyhow!("File {:?} not found on disk", file_path));
        }

        println!("{} file: {}", "opening".green().bold(), file_path.display());
        open_path(&file_path)?;
        return Ok(());
    }

    if let Ok(subject_id) = ctx.storage.get_subject_id_by_name_ci(&args.target) {
        let subject_name = ctx.storage.get_subject_name_by_id(subject_id)?;
        let subject_path = subjects_root.join(&subject_name);

        if !subject_path.is_dir() {
            return Err(anyhow!(
                "Subject folder {:?} not found on disk",
                subject_path
            ));
        }

        println!(
            "{} subject folder: {}",
            "opening".green().bold(),
            subject_path.display()
        );
        open_path(&subject_path)?;
        return Ok(());
    }

    Err(anyhow!("File or subject '{}' not found", args.target))
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

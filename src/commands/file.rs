use crate::context::AppContext;
use anyhow::{Context, Result, anyhow};
use clap::Subcommand;
use colored::Colorize;
use std::fs;
use std::path::Path;

#[derive(Subcommand)]
pub enum FileCommands {
    Add {
        subject: String,
        path: String,
    },
    Delete {
        file_name: String,
    },
    Rename {
        file_name: String,
        new_file_name: String,
    },
}

pub fn handle(ctx: &AppContext, cmd: &FileCommands) -> Result<()> {
    match cmd {
        FileCommands::Add { subject, path } => add_file(&ctx, subject, path)?,
        FileCommands::Delete { file_name } => delete_file(&ctx, file_name)?,
        FileCommands::Rename {
            file_name,
            new_file_name,
        } => rename_file(&ctx, file_name, new_file_name)?,
    }
    Ok(())
}
pub fn delete_file(ctx: &AppContext, file_name: &str) -> Result<()> {
    let subject_id = ctx.storage.get_subject_id_by_filename(file_name)?;

    let subject_name = ctx.storage.get_subject_name_by_id(subject_id)?;

    let file_path = ctx.config.subjects_path.join(&subject_name).join(file_name);

    ctx.storage.delete_file(file_name)?;

    fs::remove_file(&file_path)
        .map_err(|e| anyhow!("failed to delete file {:?}: {}", file_path, e))?;

    println!(
        "{} file '{}' from subject '{}'",
        "deleted".bold().red(),
        file_name,
        subject_name
    );

    Ok(())
}
pub fn rename_file(ctx: &AppContext, file_name: &str, new_file_name: &str) -> Result<()> {
    let subject_id = ctx.storage.get_subject_id_by_filename(file_name)?;

    let subject_name = ctx.storage.get_subject_name_by_id(subject_id)?;

    let old_path = ctx.config.subjects_path.join(&subject_name).join(file_name);
    let new_path = ctx
        .config
        .subjects_path
        .join(&subject_name)
        .join(new_file_name);

    ctx.storage.rename_file(file_name, new_file_name)?;

    fs::rename(&old_path, &new_path).map_err(|e| {
        anyhow!(
            "failed to rename file {:?} to {:?}: {}",
            old_path,
            new_path,
            e
        )
    })?;

    println!(
        "{} file '{}' to '{}'",
        "renamed".bold().blue(),
        file_name,
        new_file_name
    );

    Ok(())
}

pub fn add_file(ctx: &AppContext, subject_name: &str, file_path: &str) -> Result<()> {
    let src = Path::new(file_path);
    if !src.exists() {
        return Err(anyhow!("source file '{}' does not exist", file_path));
    }

    let subject_id = ctx
        .storage
        .get_subject_id_by_name(subject_name)
        .map_err(|_| anyhow!("subject '{}' not found", subject_name))?;

    let file_name = src
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    ctx.storage.add_file(subject_id, &file_name)?;

    let dst = Path::new(&ctx.config.subjects_path)
        .join(subject_name)
        .join(&file_name);

    fs::copy(src, &dst).with_context(|| format!("failed to copy file to {:?}", dst))?;

    println!(
        "{} file '{}' to subject '{}'",
        "added".bold().green(),
        file_name,
        subject_name
    );

    Ok(())
}

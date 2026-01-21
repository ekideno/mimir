use crate::context::AppContext;
use anyhow::{Context, Result, anyhow};
use clap::Subcommand;
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
        .map_err(|e| anyhow!("Failed to delete file {:?}: {}", file_path, e))?;

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

    if new_path.exists() {
        return Err(anyhow!("File '{}' already exists", new_file_name));
    }

    fs::rename(&old_path, &new_path).map_err(|e| {
        anyhow!(
            "Failed to rename file {:?} to {:?}: {}",
            old_path,
            new_path,
            e
        )
    })?;

    ctx.storage.rename_file(file_name, new_file_name)?;

    Ok(())
}
pub fn add_file(ctx: &AppContext, subject_name: &str, file_path: &str) -> Result<()> {
    let src = Path::new(file_path);
    if !src.exists() {
        return Err(anyhow!("Source file '{}' does not exist", file_path));
    }

    let file_name = src
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let dst = Path::new(&ctx.config.subjects_path)
        .join(subject_name)
        .join(
            src.file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("unknown")),
        );

    ctx.storage.add_file(subject_name, &file_name)?;
    fs::copy(src, &dst).with_context(|| format!("failed to copy file to {:?}", dst))?;

    Ok(())
}

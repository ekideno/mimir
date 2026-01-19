use crate::context::AppContext;
use anyhow::{Context, Result, anyhow};
use clap::Subcommand;
use std::fs;
use std::path::Path;

#[derive(Subcommand)]
pub enum FileCommands {
    Add { subject: String, path: String },
}

pub fn handle(ctx: &AppContext, cmd: &FileCommands) -> Result<()> {
    match cmd {
        FileCommands::Add { subject, path } => add_file(&ctx, subject, path)?,
    }
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

    update_zsh_completion();

    Ok(())
}
use clap::CommandFactory;
use clap_complete::{generate_to, shells::Zsh};
use std::path::PathBuf;

pub fn update_zsh_completion() {
    let mut cmd = crate::cli::Cli::command();

    let home = match std::env::var("HOME") {
        Ok(h) => PathBuf::from(h),
        Err(_) => {
            eprintln!("HOME not set, cannot install zsh completion");
            return;
        }
    };

    let completion_dir = home.join(".zsh").join("completions");

    if let Err(e) = std::fs::create_dir_all(&completion_dir) {
        eprintln!("Failed to create zsh completion dir: {}", e);
        return;
    }

    if let Err(e) = generate_to(Zsh, &mut cmd, "mimir", &completion_dir) {
        eprintln!("Failed to generate zsh completion: {}", e);
        return;
    }

    println!("✓ Zsh completion updated");
}

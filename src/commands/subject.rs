use crate::context::AppContext;
use ::std::path::Path;
use anyhow::{Context, Result};
use clap::Subcommand;
use std::fs;

#[derive(Subcommand)]
pub enum SubjectCommands {
    Add {
        subject: String,
    },
    Delete {
        subject: String,
    },
    Rename {
        subject_name: String,
        new_name: String,
    },
}

pub fn handle(ctx: &AppContext, cmd: &SubjectCommands) -> Result<()> {
    match cmd {
        SubjectCommands::Add {
            subject: subject_name,
        } => add_subject(ctx, subject_name)?,
        SubjectCommands::Delete {
            subject: subject_name,
        } => delete_subject(ctx, subject_name)?,
        SubjectCommands::Rename {
            subject_name,
            new_name: new_subject_name,
        } => rename_subject(ctx, subject_name, new_subject_name)?,
    }
    Ok(())
}

fn rename_subject(ctx: &AppContext, subject_name: &str, new_subject_name: &str) -> Result<()> {
    let base_path = ctx.config.subjects_path.clone();
    let old_path = base_path.join(subject_name);
    let new_path = base_path.join(new_subject_name);

    if !old_path.exists() {
        anyhow::bail!("folder for subject '{}' does not exist", subject_name);
    }

    fs::rename(&old_path, &new_path).with_context(|| {
        format!(
            "failed to rename folder '{}' to '{}'",
            subject_name, new_subject_name
        )
    })?;

    if let Err(e) = ctx.storage.rename_subject(subject_name, new_subject_name) {
        let _ = fs::rename(&new_path, &old_path);
        return Err(e.into());
    }

    Ok(())
}
fn delete_subject(ctx: &AppContext, subject_name: &str) -> Result<()> {
    println!("Deleting subject: {}", subject_name);
    let subject_dir = Path::new(&ctx.config.subjects_path).join(subject_name);

    ctx.storage.delete_subject(subject_name)?;

    if subject_dir.exists() {
        fs::remove_dir_all(&subject_dir).context("failed to delete subject directory")?;
    }
    Ok(())
}

fn add_subject(ctx: &AppContext, subject_name: &str) -> Result<()> {
    println!("Adding subject: {}", subject_name);

    let subject_dir = Path::new(&ctx.config.subjects_path).join(subject_name);

    if !subject_dir.exists() {
        fs::create_dir(&subject_dir).context("failed to create subject directory")?;
    }

    ctx.storage.add_subject(subject_name)?;

    println!("✓ Subject '{}' added successfully!", subject_name);
    Ok(())
}

use crate::context::AppContext;
use crate::models::Subject;
use ::std::path::Path;
use anyhow::{Context, Result};
use clap::Subcommand;
use std::fs;

#[derive(Subcommand)]
pub enum SubjectCommands {
    Add { subject: String },
}

pub fn handle(ctx: &AppContext, cmd: &SubjectCommands) -> Result<()> {
    match cmd {
        SubjectCommands::Add { subject } => add_subject(ctx, subject)?,
    }
    Ok(())
}

fn add_subject(ctx: &AppContext, subject_name: &str) -> Result<()> {
    println!("Adding subject: {}", subject_name);

    let subject_dir = Path::new(&ctx.config.subjects_path).join(subject_name);

    if !subject_dir.exists() {
        fs::create_dir(&subject_dir).context("Failed to create subject directory")?;
    }

    let subject = Subject::new(subject_name.to_string());

    ctx.storage.add_subject(subject)?;

    println!("✓ Subject '{}' added successfully!", subject_name);
    Ok(())
}

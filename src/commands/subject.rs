use crate::context::AppContext;
use crate::models::Subject;
use ::std::path::Path;
use anyhow::{Result, anyhow};
use clap::Subcommand;
use std::fs;

#[derive(Subcommand)]
pub enum SubjectCommands {
    Add { subject: String },
}

pub fn handle(ctx: &AppContext, cmd: &SubjectCommands) {
    if let Err(e) = match cmd {
        SubjectCommands::Add { subject } => add_subject(ctx, subject),
    } {
        eprintln!("Error: {}", e);
    }
}

fn add_subject(ctx: &AppContext, subject_name: &str) -> Result<()> {
    println!("Adding subject: {}", subject_name);

    // Папка предмета внутри существующей папки subjects_path
    let subject_dir = Path::new(&ctx.config.subjects_path).join(subject_name);

    // Создаём только если нет
    if !subject_dir.exists() {
        fs::create_dir(&subject_dir)
            .map_err(|e| anyhow!("Failed to create subject directory: {}", e))?;
    }

    let subject = Subject::new(subject_name.to_string());

    ctx.storage.add_subject_names(subject);

    println!("✓ Subject '{}' added successfully!", subject_name);
    Ok(())
}

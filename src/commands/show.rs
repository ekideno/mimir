use crate::context::AppContext;
use anyhow::{Context, Result};
use clap::Args;
#[derive(Args)]
pub struct ShowArgs {
    /// Name of the subject to show, or "subjects" to list all subjects
    pub name: Option<String>,
}
pub fn handle(ctx: &AppContext, args: &ShowArgs) -> Result<()> {
    match &args.name {
        None => {
            let subjects = ctx.storage.get_all_subjects_with_files()?;

            println!("Subjects and files:");
            for subject in subjects {
                println!("- {}:", subject.name);
                for file in &subject.files {
                    println!("    {}", file);
                }
            }
        }
        Some(name) if name == "subjects" => {
            let subjects = ctx.storage.get_all_subjects()?;

            println!("Subjects:");
            for subject in subjects {
                println!("- {}", subject.name);
            }
        }
        Some(subject_name) => {
            let subject = ctx
                .storage
                .get_subject(subject_name)
                .context("Failed to read subject data")?;
            println!("Files in '{}':", subject.name);
            for file in &subject.files {
                println!("- {}", file);
            }
        }
    }
    Ok(())
}

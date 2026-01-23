use crate::context::AppContext;
use crate::utils::open_path;
use anyhow::{Result, anyhow};
use clap::Args;
use colored::*;

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

    if let Ok(subject_id) = ctx.storage.get_subject_id_by_name(&args.target) {
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

use crate::context::AppContext;
use anyhow::{Result, anyhow};
use clap::Args;
use colored::Colorize;

#[derive(Args, Debug)]
pub struct FilesArgs {
    /// Name of the subject to show files for
    pub subject_name: Option<String>,
}
pub fn handle(ctx: &AppContext, args: &FilesArgs) -> Result<()> {
    if let Some(subject_name_input) = &args.subject_name {
        let subject_id = ctx
            .storage
            .get_subject_id_by_name(subject_name_input)
            .map_err(|_| anyhow!("subject '{}' not found", subject_name_input))?;

        let subject_name = ctx.storage.get_subject_name_by_id(subject_id)?;
        let files = ctx
            .storage
            .get_files_by_subject_id(subject_id)
            .map_err(|_| anyhow!("failed to get files for '{}'", subject_name))?;

        if files.is_empty() {
            println!("no files for subject '{}'", subject_name);
            return Ok(());
        }

        println!("{}", subject_name.bold());
        for (i, file_name) in files.iter().enumerate() {
            let branch = if i == files.len() - 1 {
                "└─"
            } else {
                "├─"
            };
            println!("{} {}", branch, file_name);
        }
    } else {
        let subjects = ctx
            .storage
            .get_all_subjects()
            .map_err(|_| anyhow!("failed to get subjects"))?;

        for subject_name in subjects {
            let subject_id = match ctx.storage.get_subject_id_by_name(&subject_name) {
                Ok(id) => id,
                Err(_) => continue,
            };

            let files = ctx
                .storage
                .get_files_by_subject_id(subject_id)
                .unwrap_or_default();

            if files.is_empty() {
                continue;
            }

            println!("{}", subject_name.bold());
            for (i, file_name) in files.iter().enumerate() {
                let branch = if i == files.len() - 1 {
                    "└─"
                } else {
                    "├─"
                };
                println!("{} {}", branch, file_name);
            }
            println!();
        }
    }

    Ok(())
}

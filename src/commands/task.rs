use crate::context::AppContext;
use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;

#[derive(Subcommand)]
pub enum TaskCommands {
    Add {
        subject_name: String,
        task_title: String,
    },
}

pub fn handle(ctx: &AppContext, cmd: &TaskCommands) -> Result<()> {
    match cmd {
        TaskCommands::Add {
            subject_name,
            task_title,
        } => add_task(ctx, subject_name, task_title)?,
    }
    Ok(())
}

fn add_task(ctx: &AppContext, subject_name: &str, task_title: &str) -> Result<()> {
    ctx.storage.add_task(subject_name, task_title)?;

    println!(
        "{} task \"{}\" to \"{}\"",
        "added".green().bold(),
        task_title,
        subject_name
    );
    Ok(())
}

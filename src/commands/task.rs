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
    Delete {
        subject_name: String,
        task_title: String,
    },
    Rename {
        subject_name: String,
        old_task_title: String,
        new_task_title: String,
    },
}

pub fn handle(ctx: &AppContext, cmd: &TaskCommands) -> Result<()> {
    match cmd {
        TaskCommands::Add {
            subject_name,
            task_title,
        } => add_task(ctx, subject_name, task_title)?,
        TaskCommands::Delete {
            subject_name,
            task_title,
        } => delete_task(ctx, subject_name, task_title)?,
        TaskCommands::Rename {
            subject_name,
            old_task_title,
            new_task_title,
        } => rename_task(ctx, subject_name, old_task_title, new_task_title)?,
    }
    Ok(())
}
fn delete_task(ctx: &AppContext, subject_name: &str, task_title: &str) -> Result<()> {
    let subject_id = ctx.storage.get_subject_id_by_name(subject_name)?;

    ctx.storage.delete_task(subject_id, task_title)?;

    println!(
        "{} task \"{}\" from \"{}\"",
        "deleted".red().bold(),
        task_title,
        subject_name
    );
    Ok(())
}
fn rename_task(
    ctx: &AppContext,
    subject_name: &str,
    old_task_title: &str,
    new_task_title: &str,
) -> Result<()> {
    let subject_id = ctx.storage.get_subject_id_by_name(subject_name)?;

    ctx.storage
        .rename_task(subject_id, old_task_title, new_task_title)?;

    println!(
        "{} task \"{}\" to \"{}\" in \"{}\"",
        "renamed".yellow().bold(),
        old_task_title,
        new_task_title,
        subject_name
    );
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

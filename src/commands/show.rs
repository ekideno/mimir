use crate::context::AppContext;
use anyhow::Result;
use clap::Args;
use colored::*;

#[derive(Args)]
pub struct ShowArgs {
    /// Name of the subject to show, or "subjects" to list all subjects
    pub target: Option<String>,
}
pub fn handle(ctx: &AppContext, args: &ShowArgs) -> Result<()> {
    match &args.target {
        None => {
            let subjects = ctx.storage.get_all_subjects()?;
            for subject in subjects {
                let (total, done) = ctx.storage.get_task_progress(&subject)?;
                println!("{} ({}/{})", subject.bold(), done, total);

                let tasks = ctx.storage.get_tasks_by_subject(&subject)?;
                let count = tasks.len();
                for (i, task) in tasks.into_iter().enumerate() {
                    let status = if task.done {
                        "[x]".green()
                    } else {
                        "[ ]".red()
                    };
                    let prefix = if i + 1 == count { "└─" } else { "├─" };
                    println!("{} {} {}", prefix, status, task.title);
                }
                println!();
            }
        }
        Some(subject_name) => {
            let (total, done) = ctx.storage.get_task_progress(subject_name)?;
            println!("{} ({}/{})", subject_name.bold(), done, total);

            let tasks = ctx.storage.get_tasks_by_subject(subject_name)?;
            let count = tasks.len();

            for (i, task) in tasks.into_iter().enumerate() {
                let status = if task.done {
                    "[x]".green()
                } else {
                    "[ ]".red()
                };
                let prefix = if i + 1 == count {
                    "└──"
                } else {
                    "├──"
                };
                println!("{} {} {}", prefix, status, task.title);
            }
        }
    }
    Ok(())
}

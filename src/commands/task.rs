use crate::context::AppContext;
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum TaskCommands {
    Add {
        subject_name: String,
        task_title: String,
    },
}

pub fn handle(ctx: &AppContext, cmd: &TaskCommands) {
    if let Err(e) = match cmd {
        TaskCommands::Add {
            subject_name,
            task_title,
        } => add_task(ctx, subject_name, task_title),
    } {
        eprintln!("Error: {}", e);
    }
}

fn add_task(ctx: &AppContext, subject_name: &str, task_title: &str) -> Result<()> {
    println!("Adding task: {}", task_title);

    ctx.storage.add_task(subject_name, task_title)?;

    println!("✓ Task '{}' added successfully!", task_title);
    Ok(())
}

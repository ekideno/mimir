use crate::models::Subject;
use crate::storage;
use clap::Subcommand;
use std::fs;

#[derive(Subcommand)]
pub enum SubjectCommands {
    Add { subject: String, tasks_count: u32 },
}

pub fn handle(cmd: &SubjectCommands) {
    if let Err(e) = match cmd {
        SubjectCommands::Add {
            subject,
            tasks_count,
        } => add_subject(subject, *tasks_count),
    } {
        eprintln!("Error: {}", e);
    }
}

fn add_subject(subject_name: &str, tasks_count: u32) -> Result<(), String> {
    println!("Adding subject: {}", subject_name);

    let subject_dir = format!("./subjects/{}", subject_name);
    fs::create_dir_all(&subject_dir)
        .map_err(|e| format!("Failed to create subject directory: {}", e))?;

    let subject = Subject::new(subject_name.to_string(), tasks_count, subject_dir);

    storage::add_subject("./test_data.json", subject)
        .map_err(|e| format!("✗ Error adding subject: {}", e))?;

    println!("✓ Subject '{}' added successfully!", subject_name);
    Ok(())
}

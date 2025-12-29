use crate::models::Subject;
use crate::storage;
use clap::Subcommand;
use std::fs;
use std::path::Path;

#[derive(Subcommand)]
pub enum FileCommands {
    Add { subject: String, path: String },
}

pub fn handle(cmd: &FileCommands) {
    if let Err(e) = match cmd {
        FileCommands::Add { subject, path } => add_file(subject, path),
    } {
        eprintln!("Error: {}", e);
    }
}

pub fn add_file(subject_name: &str, file_path: &str) -> Result<(), String> {
    let mut subject: Subject = storage::find_subject("test_data.json", subject_name)
        .map_err(|e| format!("Failed to read subject: {}", e))?
        .ok_or_else(|| format!("Subject '{}' not found", subject_name))?;

    let src = Path::new(file_path);
    if !src.exists() {
        return Err(format!("Source file '{}' does not exist", file_path));
    }

    let dst = Path::new(&subject.subject_dir).join(
        src.file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("unknown")),
    );

    let file_name = src
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    subject.files.push(file_name.clone());

    storage::update_subject("test_data.json", subject)
        .map_err(|e| format!("Failed to update subject: {}", e))?;

    fs::copy(src, &dst).map_err(|e| format!("Failed to copy file to {:?}: {}", dst, e))?;

    println!("Copied '{}' to {:?}", file_path, dst);
    Ok(())
}

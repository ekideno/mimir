use crate::models::Subject;
use crate::storage;
use std::fs;

pub fn execute(
    subject: Option<&str>,
    subject_name: Option<&str>,
    tasks_count: u32,
    storage_path: &str,
    file: bool,
    file_path: Option<&str>,
) {
    if let Some(subject_name) = subject {
        println!("Adding subject: {}", subject_name);

        let subject_dir = format!("./subjects/{}", subject_name);
        fs::create_dir_all(&subject_dir).expect("Failed to create subject directory");
        let subject = Subject::new(subject_name.to_string(), tasks_count, subject_dir);

        match storage::add_subject(storage_path, subject) {
            Ok(_) => {
                println!("✓ Subject '{}' added successfully!", subject_name);
            }
            Err(e) => {
                eprintln!("✗ Error adding subject: {}", e);
                std::process::exit(1);
            }
        }
    } else if file {
        if let Some(subject_name) = subject_name {
            match storage::find_subject(storage_path, subject_name) {
                Ok(Some(subject)) => {
                    println!("Subject: {}", subject.name);
                    if let Some(from_path) = file_path {
                        let _ = fs::copy(from_path, subject.subject_dir);
                    }
                }
                Ok(None) => {
                    eprintln!("Subject '{}' not found", subject_name);
                    std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

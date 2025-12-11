use crate::storage;

pub fn execute(
    show_subjects: bool,
    show_files: bool,
    subject_name: Option<&str>,
    storage_path: &str,
) {
    if show_subjects {
        show_all_subjects(storage_path);
    } else if let Some(name) = subject_name {
        if show_files {
            show_subject_files(name);
        } else {
            show_subject_info(name, storage_path);
        }
    } else if show_files && show_subjects {
        println!("Files:");
        // TODO: Implement files listing
    } else {
        eprintln!("Please specify what to show. Use --help for more information.");
        std::process::exit(1);
    }
}

fn show_all_subjects(storage_path: &str) {
    match storage::get_all_subjects(storage_path) {
        Ok(subjects) => {
            if subjects.is_empty() {
                println!("No subjects found. Add one with: mimir add --subject <name>");
            } else {
                println!("Subjects:");
                for subject in subjects {
                    println!("  • {}", subject.name);
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading subjects: {}", e);
            std::process::exit(1);
        }
    }
}

fn show_subject_info(name: &str, storage_path: &str) {
    match storage::find_subject(storage_path, name) {
        Ok(Some(subject)) => {
            println!("Subject: {}", subject.name);
            // TODO: Show more info (files, notes, etc.)
        }
        Ok(None) => {
            eprintln!("Subject '{}' not found", name);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn show_subject_files(name: &str) {
    println!("Files for \"{}\"", name);
    // TODO: Implement file listing
}

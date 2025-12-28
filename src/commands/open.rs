use crate::storage;
use std::process::Command;

pub fn execute(subject_name: Option<&str>, storage_path: &str) {
    if let Some(name) = subject_name {
        match storage::find_subject(storage_path, name) {
            Ok(Some(subject)) => {
                println!("Subject found: {}", subject.name);
                open_in_file_manager(&subject.subject_dir);
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
    } else {
        open_in_file_manager("./subjects");
    }
}

pub fn open_in_file_manager(path: &str) {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .expect("Failed to open folder in Finder");
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(path)
            .spawn()
            .expect("Failed to open folder in Explorer");
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .expect("Failed to open folder in Linux file manager");
    }
}

use crate::storage;
use std::fs;
use std::path::Path;

pub fn execute(add: bool, delete: bool, subject_name: Option<&str>, file_path: Option<&str>) {
    let (subject_name, file_path) = match (subject_name, file_path) {
        (Some(n), Some(f)) => (n, f),
        _ => return,
    };

    if add {
        add_file(subject_name, file_path);
    }

    if delete {}
}

fn add_file(subject_name: &str, file_path: &str) {
    let mut subject = match storage::find_subject("test_data.json", subject_name) {
        Ok(Some(s)) => s,
        _ => {
            eprintln!("Subject '{}' not found or error", subject_name);
            return;
        }
    };

    let src = Path::new(file_path);
    let dst = Path::new(&subject.subject_dir).join(
        src.file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("unknown")),
    );

    subject.files.push(
        src.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or("unknown".to_string()),
    );
    if let Err(e) = storage::update_subject("test_data.json", subject) {
        eprintln!(" failed: {}", e);
    }

    if let Err(e) = fs::copy(src, &dst) {
        eprintln!("Copy failed: {}", e);
    } else {
        println!("Copied to {:?}", dst);
    }
}

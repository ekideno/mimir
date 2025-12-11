use crate::models::Subject;
use crate::storage;

pub fn execute(subject_name: &str, storage_path: &str) {
    println!("Adding subject: {}", subject_name);

    let subject = Subject::new(subject_name.to_string());

    match storage::add_subject(storage_path, subject) {
        Ok(_) => {
            println!("✓ Subject '{}' added successfully!", subject_name);
        }
        Err(e) => {
            eprintln!("✗ Error adding subject: {}", e);
            std::process::exit(1);
        }
    }
}

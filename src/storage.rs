use crate::models::Subject;
use std::fs;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn load_subjects(path: &str) -> Result<Vec<Subject>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(path)?;
    let subjects = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
    Ok(subjects)
}

pub fn save_subjects(path: &str, subjects: &[Subject]) -> Result<()> {
    let json = serde_json::to_string_pretty(subjects)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn add_subject(path: &str, subject: Subject) -> Result<()> {
    let mut subjects = load_subjects(path)?;

    if subjects.iter().any(|s| s.name == subject.name) {
        return Err(format!("Subject '{}' already exists", subject.name).into());
    }

    subjects.push(subject);
    save_subjects(path, &subjects)?;
    Ok(())
}

pub fn get_all_subjects(path: &str) -> Result<Vec<Subject>> {
    load_subjects(path)
}

pub fn find_subject(path: &str, name: &str) -> Result<Option<Subject>> {
    let subjects = load_subjects(path)?;
    Ok(subjects.into_iter().find(|s| s.name == name))
}

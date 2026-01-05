use crate::models::Subject;
use std::{fs, path::Path};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn load_subjects(path: &Path) -> Result<Vec<Subject>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(path)?;
    let subjects = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
    Ok(subjects)
}

pub fn save_subjects(path: &Path, subjects: &[Subject]) -> Result<()> {
    let json = serde_json::to_string_pretty(subjects)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn add_subject(path: &Path, subject: Subject) -> Result<()> {
    let mut subjects = load_subjects(path)?;

    if subjects.iter().any(|s| s.name == subject.name) {
        return Err(format!("Subject '{}' already exists", subject.name).into());
    }

    subjects.push(subject);
    save_subjects(path, &subjects)?;
    Ok(())
}

pub fn get_all_subjects(path: &Path) -> Result<Vec<Subject>> {
    load_subjects(path)
}

pub fn find_subject(path: &Path, name: &str) -> Result<Option<Subject>> {
    let subjects = load_subjects(path)?;
    Ok(subjects.into_iter().find(|s| s.name == name))
}

/// Новая функция: обновление существующего Subject
pub fn update_subject(path: &Path, updated: Subject) -> Result<()> {
    let mut subjects = load_subjects(path)?;
    let mut found = false;

    for subject in &mut subjects {
        if subject.name == updated.name {
            // Обновляем все поля
            subject.files = updated.files.clone();
            subject.subject_dir = updated.subject_dir.clone();
            // Если есть другие поля — обновляем здесь
            found = true;
            break;
        }
    }

    if !found {
        return Err(format!("Subject '{}' not found", updated.name).into());
    }

    save_subjects(path, &subjects)?;
    Ok(())
}

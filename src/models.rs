use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subject {
    pub name: String,
    pub task_count: u32,
    pub subject_dir: String,
    pub files: Vec<String>,
}

impl Subject {
    pub fn new(name: String, task_count: u32, subject_dir: String) -> Self {
        Self {
            name,
            task_count,
            subject_dir,
            files: Vec::new(),
        }
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct File {
//     pub name: String,
//     pub subject: String,
//     pub path: String,
// }

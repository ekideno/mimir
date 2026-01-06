pub struct Subject {
    pub name: String,
    pub task_count: u32,
    pub files: Vec<String>,
}

impl Subject {
    pub fn new(name: String, task_count: u32) -> Self {
        Self {
            name,
            task_count,
            files: Vec::new(),
        }
    }
}

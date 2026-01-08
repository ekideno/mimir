pub struct Subject {
    pub name: String,
    pub files: Vec<String>,
}

impl Subject {
    pub fn new(name: String) -> Self {
        Self {
            name,
            files: Vec::new(),
        }
    }
}

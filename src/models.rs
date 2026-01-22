pub struct Subject {
    pub name: String,
    pub files: Vec<String>,
}

impl Subject {}

pub struct Task {
    pub title: String, // Название задачи
    pub done: bool,    // Статус выполнения: true = выполнено, false = нет
}

use crate::storage;

pub fn handle(scope: &str, prefix: &str) {
    let mut results = Vec::new();

    if scope == "open" {
        if let Ok(subjects) = storage::get_all_subjects("test_data.json") {
            let prefix_lc = prefix.to_lowercase(); // приводим префикс к нижнему регистру

            for subject in subjects {
                // приводим имя subject к нижнему регистру для сравнения
                if subject.name.to_lowercase().starts_with(&prefix_lc) {
                    results.push(subject.name.clone());
                }

                for file_name in &subject.files {
                    if file_name.to_lowercase().starts_with(&prefix_lc) {
                        results.push(format!("{}", file_name));
                    }
                }
            }
        }
    }

    for r in results {
        println!("{}", r);
    }
}

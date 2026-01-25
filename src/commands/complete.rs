use crate::context::AppContext;
use std::collections::HashSet;

pub fn handle(ctx: &AppContext, scope: &str, prefix: &str) {
    if scope != "open" {
        return;
    }

    let prefix_lc = prefix.to_lowercase();
    let mut seen = HashSet::new();

    if let Ok(subjects) = ctx.storage.get_all_subjects_with_files() {
        for subject in subjects {
            if subject.name.to_lowercase().starts_with(&prefix_lc) {
                if seen.insert(subject.name.clone()) {
                    println!("{}", subject.name);
                }
            }

            for file in subject.files {
                if file.to_lowercase().starts_with(&prefix_lc) {
                    if seen.insert(file.clone()) {
                        println!("{}", file);
                    }
                }
            }
        }
    }
}

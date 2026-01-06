use crate::context::AppContext;
use clap::Args;

#[derive(Args)]
pub struct ShowArgs {
    /// Name of the subject to show, or "subjects" to list all subjects
    pub name: Option<String>,
}
pub fn handle(ctx: &AppContext, args: &ShowArgs) {
    match &args.name {
        None => {
            if let Ok(subjects) = ctx.storage.get_all_subjects_with_files() {
                println!("Subjects and files:");
                for subject in subjects {
                    println!("- {}:", subject.name);
                    for file in &subject.files {
                        println!("    {}", file);
                    }
                }
            } else {
                eprintln!("Failed to read subjects data");
            }
        }
        Some(name) if name == "subjects" => {
            if let Ok(subjects) = ctx.storage.get_all_subjects() {
                println!("Subjects:");
                for subject in subjects {
                    println!("- {}", subject.name);
                }
            } else {
                eprintln!("Failed to read subjects data");
            }
        }
        Some(subject_name) => {
            if let Ok(subject) = ctx.storage.get_subject(subject_name) {
                println!("Files in '{}':", subject.name);
                for file in &subject.files {
                    println!("- {}", file);
                }
            }
        }
    }
}

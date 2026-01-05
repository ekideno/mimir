use crate::context::AppContext;
use crate::storage;
use clap::Args;

#[derive(Args)]
pub struct ShowArgs {
    /// Name of the subject to show, or "subjects" to list all subjects
    pub name: Option<String>,
}
pub fn handle(ctx: &AppContext, args: &ShowArgs) {
    match &args.name {
        None => {
            if let Ok(subjects) = storage::get_all_subjects(&ctx.config.data_path) {
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
            if let Ok(subjects) = storage::get_all_subjects(&ctx.config.data_path) {
                println!("Subjects:");
                for subject in subjects {
                    println!("- {}", subject.name);
                }
            } else {
                eprintln!("Failed to read subjects data");
            }
        }
        Some(subject_name) => match storage::find_subject(&ctx.config.data_path, subject_name) {
            Ok(Some(subject)) => {
                println!("Files in '{}':", subject.name);
                for file in &subject.files {
                    println!("- {}", file);
                }
            }
            Ok(None) => eprintln!("Subject '{}' not found", subject_name),
            Err(e) => eprintln!("Failed to read data: {}", e),
        },
    }
}

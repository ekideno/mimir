mod cli;
mod commands;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let storage_path = "test_data.json";

    match &cli.command {
        Commands::File {
            add,
            delete,
            subject_name,
            file_path,
        } => commands::file::execute(*add, *delete, subject_name.as_deref(), file_path.as_deref()),
        Commands::Add {
            subject,
            subject_name,
            tasks_count,
            file,
            file_path,
        } => {
            commands::add::execute(
                subject.as_deref(),
                subject_name.as_deref(),
                *tasks_count,
                storage_path,
                *file,
                file_path.as_deref(),
            );
        }
        Commands::Show {
            subjects,
            files,
            subject_name,
        } => {
            commands::show::execute(*subjects, *files, subject_name.as_deref(), storage_path);
        }
        Commands::Open { subject_name } => {
            commands::open::execute(subject_name.as_deref(), storage_path);
        }
    }
}

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
        Commands::Add { subject } => {
            commands::add::execute(subject, storage_path);
        }
        Commands::Show {
            subjects,
            files,
            subject_name,
        } => {
            commands::show::execute(*subjects, *files, subject_name.as_deref(), storage_path);
        }
    }
}

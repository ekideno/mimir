use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mimir")]
#[command(about = "Study Manager CLI")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new subject
    Add {
        /// Name of the subject
        #[arg(short, long)]
        subject: String,
    },
    /// Show subjects or files
    Show {
        /// Show all subjects
        #[arg(short, long)]
        subjects: bool,
        /// Show files
        #[arg(short, long)]
        files: bool,
        /// Optional: name of a specific subject
        subject_name: Option<String>,
    },
}

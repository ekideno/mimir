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
    // TODO: rename flag --rename
    // mimir file --add --delete <subject> <file_path>
    /// Work with file for subject
    File {
        #[arg(short, long)]
        add: bool,
        #[arg(short, long)]
        delete: bool,

        subject_name: Option<String>,
        file_path: Option<String>,
    },

    // TODO: replace with mimir subject --add --delete --rename <subject_name>
    /// Add a new subject
    Add {
        /// Name of the subject
        #[arg(short, long)]
        subject: Option<String>,
        tasks_count: u32,

        #[arg(short, long)]
        file: bool,

        subject_name: Option<String>,

        file_path: Option<String>,
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

    Open {
        subject_name: Option<String>,
    },
}

// Commands
// mimir file --add --delete <subject> <file_path>
// mimir subject --add --delete --rename <subject_name>
// mimir show <subject_name>
// mimir show <file_name>
//
// mimir open <subject_name> or mimir <subject_name>
// mimir open <file_name> or mimir <file_name>

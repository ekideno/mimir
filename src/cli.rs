use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mimir")]
#[command(about = "Study Manager CLI")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional positional argument for "open" shortcut
    #[arg(value_name = "search_name", required = false)]
    pub default_open: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Subject(super::commands::subject::SubjectCommands),

    #[command(subcommand)]
    File(super::commands::file::FileCommands),

    Open(super::commands::open::OpenArgs),
    Show(super::commands::show::ShowArgs),

    #[command(hide = true)]
    __Complete {
        scope: String,
        prefix: String,
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

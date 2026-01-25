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

    #[command(subcommand)]
    Task(super::commands::task::TaskCommands),

    Open(super::commands::open::OpenArgs),
    Tasks(super::commands::tasks::TasksArgs),

    Files(super::commands::files::FilesArgs),
    Workspace,
    Config,

    Completions {
        #[arg(value_enum)]
        shell: super::commands::completions::Shell,
    },
    #[command(hide = true)]
    __Complete {
        scope: String,
        prefix: String,
    },
}

mod cli;
mod commands;
mod models;
mod storage;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};
use commands::file;

use crate::commands::{complete, open, show, subject};

fn main() {
    let cli = Cli::parse();

    if cli.default_open.is_none() && cli.command.is_none() {
        Cli::command().print_help().unwrap();
        println!();
        return;
    }
    // if let Some(name) = cli.default_open {
    //     open::handle(&open::OpenArgs { name });
    //     return;
    // }

    if let Some(command) = cli.command {
        match command {
            Commands::__Complete { scope, prefix } => complete::handle(&scope, &prefix),
            Commands::Open(args) => open::handle(&args),
            Commands::Show(args) => show::handle(&args),
            Commands::File(cmd) => file::handle(&cmd),
            Commands::Subject(cmd) => subject::handle(&cmd),
        }
    }
}

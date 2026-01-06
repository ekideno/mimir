mod cli;
mod commands;
mod config;
mod context;
mod models;
mod storage;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};
use commands::file;
use context::AppContext;

use crate::commands::{complete, open, show, subject};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.default_open.is_none() && cli.command.is_none() {
        Cli::command().print_help().unwrap();
        return Ok(());
    }

    let ctx = AppContext::init()?;

    if let Some(name) = cli.default_open {
        open::handle(&ctx, &open::OpenArgs { name });
        return Ok(());
    }
    if let Some(command) = cli.command {
        match command {
            Commands::__Complete { scope, prefix } => complete::handle(&ctx, &scope, &prefix),
            Commands::Open(args) => open::handle(&ctx, &args),
            Commands::Show(args) => show::handle(&ctx, &args),
            Commands::File(cmd) => file::handle(&ctx, &cmd),
            Commands::Subject(cmd) => subject::handle(&ctx, &cmd),
        }
    }

    Ok(())
}

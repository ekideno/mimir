mod cli;
mod commands;
mod config;
mod context;
mod errors;
mod models;
mod storage;
mod utils;
use crate::commands::config as cmd_config;
use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};
use colored::*;
use context::AppContext;

use crate::commands::{complete, file, files, open, show, subject, task, workspace};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}: {:?}", "error".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.default_open.is_none() && cli.command.is_none() {
        Cli::command().print_help().unwrap();
        return Ok(());
    }

    let ctx = AppContext::init()?;

    if let Some(name) = cli.default_open {
        open::handle(&ctx, &open::OpenArgs { target: name })?;
        return Ok(());
    }
    if let Some(command) = cli.command {
        match command {
            Commands::__Complete { scope, prefix } => complete::handle(&ctx, &scope, &prefix),
            Commands::Open(args) => open::handle(&ctx, &args)?,
            Commands::Show(args) => show::handle(&ctx, &args)?,
            Commands::File(cmd) => file::handle(&ctx, &cmd)?,
            Commands::Subject(cmd) => subject::handle(&ctx, &cmd)?,
            Commands::Task(cmd) => task::handle(&ctx, &cmd)?,
            Commands::Files(args) => files::handle(&ctx, &args)?,
            Commands::Workspace => workspace::handle(&ctx)?,
            Commands::Config => cmd_config::handle(&ctx)?,
        }
    }

    Ok(())
}

mod commands;
mod config;
mod db;
use anyhow::{Ok, Result};
use clap::Parser;
mod cli;
use cli::{Cli, CliCommand};
use log::{info, LevelFilter};

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_logging();
    match cli.command {
        CliCommand::Init(init_args) => commands::run(CliCommand::Init(init_args)),
        CliCommand::Task(task_args) => todo!(),
        CliCommand::Export(export_args) => todo!(),
    }?;
    Ok(())
}

fn init_logging() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .default_format()
        .format_target(false)
        .init();
}

use anyhow::Result;

use crate::cli::{Cli, CliCommand};
pub mod init;
pub fn run(command: CliCommand) -> Result<()> {
    match command {
        crate::cli::CliCommand::Init(init_args) => init::run(),
        crate::cli::CliCommand::Task(task_args) => todo!(),
        crate::cli::CliCommand::Export(export_args) => todo!(),
    }?;
    Ok(())
}

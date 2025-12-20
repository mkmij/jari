mod commands;
use anyhow::{Ok, Result};
use clap::Parser;
mod cli;
use cli::{Cli};

fn main() -> Result<()> {
    let _cli = Cli::parse();
    Ok(())
//     match cli.command {
//          => {},
// }
    // match cli.command {
    //     Commands::Init(_init) => todo!(),
    //     Commands::Task(task) => {
    //         let TaskArgs { command: _, filter } = task;
    //         filter.iter().filter(|val| val.is_some())
    //             .map(|arg| arg.as_ref().unwrap())
    //             .for_each(|arg| {
    //                 println!("{} -> {}", arg.0, arg.1);
    //         });
    //     }
    // }
}

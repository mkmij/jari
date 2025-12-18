mod commands;
use anyhow::Result;
use clap::Parser;
use jari::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let _cli = Cli::parse();
    Ok(())
    // let args: Vec<String> = args().collect();
    // for arg in args {
    //     println!("{arg}");
    // }
    // println!("{}", handle_help());
    // match handle_init() {
    //     Ok(_) => println!("Successfully initialized jari in this project."),
    //     Err(e) => eprintln!("Error: {e}"),
    // }
    // Ok(())
}

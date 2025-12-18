pub mod cli {
    use clap::{Args, Parser, Subcommand};
    use std::fmt::Debug;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about)]
    pub struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand, Debug)]
    enum Commands {
        #[command(about = "Initialize jari in a project", long_about = None)]
        Init(InitArgs),
        List,
        Task,
    }

    #[derive(Args, Debug)]
    struct InitArgs {
        #[arg(
            short,
            long,
            help = "Specify path to folder in which to initialize jari. Absolute or relative paths supported",
            default_value = "CURRENT_DIR_PATH"
        )]
        path: Option<String>,
        #[arg(short, long, help = "Specify the name of the project",
            default_values = ["CURRENT_DIR_PATH_NAME", "|", "SUPPLIED_DIR_PATH_NAME"])]
        name: Option<String>,
    }
}

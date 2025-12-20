use clap::builder::PossibleValue;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    ///Initialize a jari project
    #[command(
        long_about = "If invoked with arguments presents a shorthand for initializing a jari project without going through interactive mode",
        visible_alias = "i"
    )]
    Init(InitArgs),
    ///Manage project tasks
    #[command(
        long_about = "When invoked without subcommands launches an interactive prompt that lists tasks in the current project",
        visible_alias = "t"
    )]
    Task(TaskArgs),
    ///Export tasks from projects
    #[command(
        long_about = "If args are supplied exports all tasks from the specified project and in the specified format without filtering",
        visible_alias = "e"
    )]
    Export(ExportArgs),
}

#[derive(Args, Debug)]
pub struct InitArgs {
    ///Specify path to directory in which to initialize jari. Absolute or relative paths supported
    #[arg(short, long, default_value = "CURRENT_DIR_PATH", value_name = "PATH")]
    pub dir: Option<String>,
    ///Specify the name of the project
    #[arg(short, long, default_values = ["CURRENT_DIR_NAME", "SUPPLIED_DIR_NAME"])]
    pub name: Option<String>,
    ///If set, initialize this jari project locally instead of globally
    #[arg(
        short,
        long,
        default_value_t = false,
        long_help = r"By default, jari stores all data related to its projects in $XDG_DATA_HOME, with the intention of minimizing clutter on the user's system.
In some cases users may wish to have their project data stored locally in the directory specified when initializing a jari project.
When init is invoked with the -l flag a .jari folder is created in the project root and jari is configured to store task information in it."
    )]
    pub local: bool,
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TaskArgs {
    #[command(subcommand)]
    pub command: Option<TaskCommand>,
    ///A quick way to update the status of a task by specifying its full title or id. Requires -s to also be provided
    #[arg(short, value_name = "PROJECT_TITLE | ID", requires = "status")]
    pub title: Option<String>,
    ///What status to change the specified project to. Requires -t to be provided
    #[arg(short, value_name = "STATUS", name = "status")]
    pub status: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum TaskCommand {
    ///Create a new task in a project
    New(NewArgs),
    ///Generate tasks from the files in a project. Respects .gitignore and ignores specified in .jari.toml
    Generate {
        ///Specify a file in the current project to generate tasks from
        #[arg(short, long, value_name = "FILENAME")]
        file: Option<String>,
    },
}

#[derive(Args, Debug)]
pub struct NewArgs {
    ///Title of the new task
    #[arg(short, long, value_name = "PROJECT_TITLE | ID")]
    pub title: String,
    ///Status of the new task. If left blank the first value from the .jari.toml file will be used
    #[arg(short, long)]
    pub status: Option<String>,
    ///Description of the new task
    #[arg(short, long)]
    pub description: Option<String>,
}

#[derive(Args, Debug)]
pub struct ExportArgs {
    ///Format of the exported tasks
    #[arg(short, long, value_name = "FORMAT", default_value = "json", value_enum)]
    pub format: Option<ExportFormat>,
    ///Name of the project to export from
    #[arg(short, long, value_name = "NAME")]
    pub project_name: Option<String>,
    // ///Filter tasks to export by project name, task title, task description or task status
    // #[arg(short, long, value_names = ["TITLE|ID", "DESC", "STATUS"], value_parser = StringValueParser::new().map(|arg| {
    //     let arg = &arg.clone();
    //     let [k, v] = <[&str; 2]>::try_from(arg.split("=").collect::<Vec<&str>>()).unwrap();
    //     match k {
    //         "title" | "description" | "status" => Some((k.to_string(), v.to_string())),
    //         _ => None,
    //     }
    // }),
    //       long_help = r#"Accepts values in the format of arg=val, where arg can be one of [title|description|status] and uses the values in a `LIKE` query to filter tasks.
    // Can be repeated, ex: -f title=foo -fdescription=bar --filter status=baz. Any other values for arg are silently ignored."#)]
    //     pub filter: Vec<Option<(String, String)>>,
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    JSON,
    CSV,
    SQL,
}

impl ValueEnum for ExportFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::JSON, Self::CSV, Self::SQL]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::JSON => PossibleValue::new("json"),
            Self::CSV => PossibleValue::new("csv"),
            Self::SQL => PossibleValue::new("sql"),
        })
    }
}

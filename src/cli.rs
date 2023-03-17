use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "schedulite")]
#[command(about = "A scheduler for light tasks", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file for the tasks to run
    #[arg(long, value_name = "FILE", default_value = "schedulite.json")]
    pub config: Option<PathBuf>,

    /// Sets a custom log file to write to
    #[arg(long, value_name = "FILE", default_value = "log4rs.yaml")]
    pub logcfg: Option<PathBuf>,
}

pub fn parse_cli() -> Cli {
    let cli = Cli::parse();

    cli
}

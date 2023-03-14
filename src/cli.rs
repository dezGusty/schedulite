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

    // // You can check the value provided by positional arguments, or option arguments

    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }
    // if let Some(log_input) = cli.logcfg.as_deref() {
    //     println!("Value for logcfg: {}", log_input.display());
    // }

    // println!("{:?}", cli);

    cli
}

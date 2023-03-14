#[cfg(windows)]
#[tokio::main]
async fn main() {
    let cli = schedulite::cli::parse_cli();

    let logger_config_file = match cli.logcfg.as_deref() {
        Some(log_input) => log_input.to_string_lossy().to_string(),
        None => "./log4rs.yaml".to_string(),
    };

    let task_config_file = match cli.config.as_deref() {
        Some(config_path) => config_path.to_string_lossy().to_string(),
        None => "./schedulite.json".to_string(),
    };

    schedulite::load_logger_config(&logger_config_file);

    schedulite::main_loop(&task_config_file);
}

#[cfg(unix)]
#[tokio::main]
async fn main() {
    schedulite::load_logger_config();

    schedulite::main_loop();
}

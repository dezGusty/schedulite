#[cfg(windows)]
#[tokio::main]
async fn main() {
    schedulite::load_logger_config();

    schedulite::main_loop();
}

#[cfg(unix)]
#[tokio::main]
async fn main() {
    schedulite::load_logger_config();

    schedulite::main_loop();
}

pub mod filecopy;

use std::time::Duration;

use clokwerk::{AsyncScheduler, TimeUnits};

pub async fn async_copy_op() {
    let source_path = String::from("./data/test-input.txt");
    let destination_path = String::from("./data/test-output.txt");
    let result = filecopy::copy_file(source_path.as_str(), destination_path.as_str());
    match result {
        Ok(_) => println!("File copied successfully"),
        Err(e) => println!("Error: {}", e),
    }
}

#[tokio::main]
async fn main() {
    // Create a scheduler
    let mut scheduler = AsyncScheduler::new();

    // Schedule a job to run every hour
    scheduler.every(1.minute()).run(|| async_copy_op());

    // Manually run the scheduler forever
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

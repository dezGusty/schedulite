//main.rs
pub mod filecopy;

use std::{fs::File, io::BufReader};

use clokwerk::{AsyncScheduler, Interval, TimeUnits};
use serde::Deserialize;
use serde_json::Error;
use tokio::time::Duration;

#[derive(Clone, Debug, Deserialize)]
pub enum TaskType {
    CopyFile,
    MoveFile,
}

#[derive(Clone, Debug, Deserialize)]
pub enum TaskFrequency {
    Every10Seconds,
    EveryMinute,
    Every10Minutes,
    EveryHour,
    EveryDay,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TaskConfig {
    task_type: TaskType,
    frequency: TaskFrequency,
    start_time: String,
    name: String,
    source_file: String,
    destination_file: String,
}

pub async fn async_simple_task<'a>(cfg: TaskConfig) {
    println!("Simple task {} ({:?})", cfg.name, cfg.task_type);
    match cfg.task_type {
        TaskType::CopyFile => async_copy_op().await,
        TaskType::MoveFile => async_move_op().await,
    }
    
}

pub async fn async_copy_op() {
    let source_path = String::from("./data/test-input.txt");
    let destination_path = String::from("./data/test-output.txt");
    let result = filecopy::copy_file(source_path.as_str(), destination_path.as_str());
    match result {
        Ok(_) => println!("File copied successfully"),
        Err(e) => println!("Error: {}", e),
    }
}

pub async fn async_move_op() {
    let source_path = String::from("./data/test-input.txt");
    let destination_path = String::from("./data/test-output.txt");
    println!("(Suspended) Moving file from {} to {}", source_path, destination_path);
}

pub fn load_task_configs_from_json(input_file: &str) -> Result<Vec<TaskConfig>, Error> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader);
    tasks
}

#[tokio::main]
async fn main() {
    let tasks: Vec<TaskConfig> = load_task_configs_from_json("./data/tasks1.json").unwrap();

    println!("{:#?}", tasks);

    // Create a scheduler
    let mut scheduler = AsyncScheduler::new();

    for task in tasks {
        // Create an Interval from a TaskFrequency
        let ival: Interval = match task.frequency {
            TaskFrequency::Every10Seconds => Interval::Seconds(10),
            TaskFrequency::EveryMinute => Interval::Minutes(1),
            TaskFrequency::Every10Minutes => Interval::Minutes(10),
            TaskFrequency::EveryHour => Interval::Hours(1),
            TaskFrequency::EveryDay => Interval::Days(1),
        };
        //TODO: also consider using the start_time.
        let _ = chrono::NaiveTime::parse_from_str(&task.start_time, "%H:%M:%S").unwrap();
        let my_clone = task.clone();

        scheduler
            .every(ival)
            .run(move || async_simple_task(my_clone.clone()));
    }

    // Manually run the scheduler forever
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

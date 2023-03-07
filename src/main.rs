//main.rs
pub mod filecopy;

use std::{fs::File, io::BufReader};

use chrono::Local;
use cron::Schedule;
use job_scheduler_ng::{Job, JobScheduler};
use log::{debug, error, info};
use std::str::FromStr;

use serde::Deserialize;
use serde_json::Error;
use tokio::{spawn, time::Duration};

#[derive(Clone, Debug, Deserialize)]
pub enum TaskType {
    CopyFile,
    MoveFile,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TaskConfig {
    task_type: TaskType,
    frequency_cron_config: String,
    name: String,
    source_file: String,
    destination_file: String,
}

pub async fn async_simple_task<'a>(cfg: TaskConfig) {
    info!("Simple task {} ({:?})", cfg.name, cfg.task_type);
    match cfg.task_type {
        TaskType::CopyFile => async_copy_op(&cfg.source_file, &cfg.destination_file).await,
        TaskType::MoveFile => async_move_op(&cfg.source_file, &cfg.destination_file).await,
    }
}

pub fn sync_simple_task_forwarder(cfg: TaskConfig) {
    tokio::task::block_in_place(|| {
        spawn(async move {
            async_simple_task(cfg.clone()).await;
        });
    })
}

pub async fn async_copy_op(source_path: &str, destination_path: &str) {
    let result = filecopy::copy_file(source_path, destination_path);
    match result {
        Ok(_) => info!(
            "File copied successfully 📜({} -> {})",
            source_path, destination_path
        ),
        Err(e) => error!("Error: {}", e),
    }
}

pub async fn async_move_op(source_path: &str, destination_path: &str) {
    debug!(
        "(Suspended) Would move file from {} to {}",
        source_path, destination_path
    );
}

pub fn load_task_configs_from_json(input_file: &str) -> Result<Vec<TaskConfig>, Error> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader);
    tasks
}

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("--- Starting up ---");
    let tasks: Vec<TaskConfig> = load_task_configs_from_json("./data/tasks1.json").unwrap();

    debug!("Started with {:#?} tasks configured", tasks.len());

    // Create a scheduler
    let mut scheduler = JobScheduler::new();

    for task in tasks {
        // Create an Interval from a TaskFrequency
        // sec  min   hour   day of month   month   day of week   year
        // 0    1     2      3              4       5             6
        // let cron_frequency: String = match task.frequency {
        //     TaskFrequency::Every10Seconds => "*/10 * * * * *".to_string(),
        //     TaskFrequency::EveryMinute => "0 * * * * *".to_string(),
        //     TaskFrequency::Every10Minutes => "0 */10 * * * *".to_string(),
        //     TaskFrequency::EveryHour => "0 0 * * * *".to_string(),
        //     TaskFrequency::EveryDay => "0 0 0 * * *".to_string(),
        //     _ => "* * * 1 * *".to_string(),
        // };

        let my_clone = task.clone();

        let localoffset = chrono::offset::Local;
        let current_millis = Local::now().timestamp_millis();
        let schedule = Schedule::from_str(&task.frequency_cron_config).unwrap();

        let next_items = schedule.upcoming(localoffset);

        for item in next_items.take(1) {
            debug!(
                "Task {}, Next fire time: {} (time from now: {}.{}s)",
                task.name,
                item,
                (item.timestamp_millis() - current_millis) / 1000,
                (item.timestamp_millis() - current_millis) % 1000
            );
        }

        scheduler.add(Job::new(schedule, move || {
            sync_simple_task_forwarder(my_clone.clone())
        }));
    }

    // Manually run the scheduler forever
    loop {
        scheduler.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}

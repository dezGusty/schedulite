//main.rs
pub mod cli;
pub mod filecopy;
pub mod ops;
pub mod tests;

use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use chrono::Local;
use cron::Schedule;
use job_scheduler_ng::{Job, JobScheduler};
use log::{debug, info, trace};
use std::str::FromStr;

use hotwatch::{Event, Hotwatch};

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
    enabled: bool,
    frequency_cron_config: String,
    run_at_startup_if_next_run_gt_s: i64,
    name: String,
    source_file: String,
    destination_file: String,
}

pub async fn async_simple_task<'a>(cfg: TaskConfig) {
    info!("Simple task {} ({:?})", cfg.name, cfg.task_type);
    match cfg.task_type {
        TaskType::CopyFile => ops::async_copy_op(&cfg.source_file, &cfg.destination_file).await,
        TaskType::MoveFile => ops::async_move_op(&cfg.source_file, &cfg.destination_file).await,
    }
}

pub fn sync_simple_task_forwarder(cfg: TaskConfig) {
    tokio::task::block_in_place(|| {
        spawn(async move {
            async_simple_task(cfg.clone()).await;
        });
    })
}

pub fn load_task_configs_from_json(input_file: &str) -> Result<Vec<TaskConfig>, Error> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader);
    tasks
}

pub fn schedule_tasks(config_tasks_file: &str, is_at_startup: bool) -> JobScheduler {
    info!(
        "🔃 (re)loading task configuration from {}",
        config_tasks_file
    );
    let tasks: Vec<TaskConfig> = load_task_configs_from_json(config_tasks_file).unwrap();

    debug!("Started with {:#?} tasks configured", tasks.len());

    // Create a scheduler
    let mut scheduler = JobScheduler::new();

    // filter out disabled tasks
    let tasks: Vec<TaskConfig> = tasks
        .into_iter()
        .filter(|task| task.enabled)
        .collect::<Vec<TaskConfig>>();
    debug!("Active tasks: {:#?}", tasks.len());
    trace!("Tasks {:#?}", tasks);

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
            let seconds_to_next_run = (item.timestamp_millis() - current_millis) / 1000;
            debug!(
                "Task {}, Next fire time: {} (time from now: {}.{}s)",
                task.name,
                item,
                seconds_to_next_run,
                (item.timestamp_millis() - current_millis) % 1000
            );

            if is_at_startup && task.run_at_startup_if_next_run_gt_s > 0 {
                // If we are starting up, check to see how much time is left until the next execution.
                // If the time until the next execution is greater than a configured treshold, we should run the task immediately
                if seconds_to_next_run > task.run_at_startup_if_next_run_gt_s {
                    info!(
                        "Emergency run ℹ️. Task {} next execution beyond run treshold: {}",
                        task.name, task.run_at_startup_if_next_run_gt_s
                    );
                    sync_simple_task_forwarder(my_clone.clone());
                }
            }
        }

        scheduler.add(Job::new(schedule, move || {
            sync_simple_task_forwarder(my_clone.clone())
        }));
    }

    scheduler
}

pub fn get_file_path_from_current_dir_or_app_dir(file_name: &str) -> Option<PathBuf> {
    let current_exe_path = std::env::current_exe().unwrap();
    let current_exe_dir = current_exe_path.parent().unwrap();

    let file_path = Path::new(file_name);
    if file_path.exists() {
        return Some(file_path.to_path_buf());
    }

    let file_path = current_exe_dir.join(file_name);
    if file_path.exists() {
        return Some(file_path);
    }

    None
}

pub fn load_logger_config(file_name: &str) {
    if let Some(logger_config_file) = get_file_path_from_current_dir_or_app_dir(file_name) {
        log4rs::init_file(&logger_config_file, Default::default()).unwrap();
    } else {
        println!("Failed to initialize logger!");
        println!("You need to have a log4rs.yaml file in the current directory or in the same directory as the executable.");
        std::process::exit(1);
    }
}

pub fn search_task_config_file(file_name: &str) -> String {
    let mut config_tasks_file = file_name.to_string();
    if let Some(test_tasks_file) = get_file_path_from_current_dir_or_app_dir(&config_tasks_file) {
        config_tasks_file = test_tasks_file.to_str().unwrap().to_string();
        info!("------ === Starting up Schedulite 🏃💨 === ------");
        debug!(
            "Will monitor config file for changes: {}",
            &config_tasks_file
        );
    } else {
        println!("Failed to load tasks!");
        println!("You need to have a [{}] file in the current directory or in the same directory as the executable.", file_name);
        std::process::exit(1);
    }

    config_tasks_file
}

pub fn monitor_file_changes_in_hotwatch(hotwatch: &mut Hotwatch, config_tasks_file: String) -> Arc<AtomicBool> {
    let config_file_changed = AtomicBool::new(false).into();
    {
        let config_file_changed = Arc::clone(&config_file_changed);
        hotwatch
            .watch(config_tasks_file, move |event: Event| {
                if let Event::Write(path) = event {
                    info!("Config file changed! {:?}", path);
                    config_file_changed.store(true, Ordering::Release);
                }
            })
            .expect("Failed to watch file!");
    }

    config_file_changed
}

pub fn main_loop(input_config_file_name: &str) {
    let config_tasks_file = search_task_config_file(input_config_file_name);

    // Set up file monitoring for the configuration file. If the file changes, we should reload the tasks
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    let config_file_changed = monitor_file_changes_in_hotwatch(&mut hotwatch, config_tasks_file.clone());

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut scheduler = schedule_tasks(&config_tasks_file, true);

    loop {
        // Manually run the scheduler until a configuration change
        let mut keep_loop = true;
        while keep_loop {
            scheduler.tick();

            if config_file_changed
                .compare_exchange_weak(true, false, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                info!("🎊 Config file has changed! Should reload tasks!");
                keep_loop = false;
            }

            if !running.load(Ordering::SeqCst) {
                info!("Got exit signal. Exiting.");
                std::process::exit(0);
            }

            std::thread::sleep(Duration::from_millis(500));
        }

        info!("Reloading tasks...");
        // Reload the tasks
        drop(scheduler);
        scheduler = schedule_tasks(&config_tasks_file, false);
    }
}

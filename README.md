# Schedulite

A simple scheduler for light tasks.

⚠️ The sample tasks are file copy operations with some replacements. To do something more than copying files, you would need to code and add that behaviour yourself.

## Features

- configuration of tasks via a json file
- option to run task at application startup if time to cron job exceeds a configurable value
- monitoring and live reloading of the configuration file
- task config file and logger config file can be loaded either from current working directory or from running binary location.
- file name replacements with custom formatting.

## Configuration

The list of tasks to run is configured via a json file.
While most fields are straightforward, the frequency is defined according to the UNIX cron format

Examples /  further reading:

- <https://www.ibm.com/docs/en/db2oc?topic=task-unix-cron-format/> .
- <https://docs.rs/job_scheduler/latest/job_scheduler/>
- <https://crates.io/crates/job_scheduler_ng>

## Misc

- what happens (how the lib treats the time if skipped) if there are one or more tasks between thread sleeps, without hitting the exact match?
main loop:

```rust
loop {
        scheduler.tick();

        std::thread::sleep(Duration::from_millis(30000)); // <--30 seconds delay
    }
```

- E.g. task1: "frequency_cron_config": `17 * * * * *`, => every minute at HH:MM:17
- E.g. task2: "frequency_cron_config": `*/10 * * * * *`, => every 10 seconds

main loop:

```rust
loop {
        scheduler.tick();

        std::thread::sleep(Duration::from_millis(30000)); // <--30 seconds delay
    }
```

The task1 shall execute once per minute, once the time elapses.
The task2 shall execute twice per minute (due to the large delay of 30seconds, which exceeds the 10seconds frequency).

However, if the sleep duration is larger than the interval.

- E.g. task1: "frequency_cron_config": `17 * * * * *`, => every minute at HH:MM:17
- E.g. task2: "frequency_cron_config": `*/10 * * * * *`, => every 10 seconds

## File copy

The file copy operation is provided as an example of a simple operation.

Nothing special is configured for the file attributes, so tt shall use the default behaviour for copying files.
What it does allow is using some placeholders for variables.

| Variable | Will be replaced with |
| :--- | :--- |
| :{DD} | Day of month (2 digits) |
| :{MM} | Month number (2 digits)
| :{YYYY} | Year (4 digits) |
| :{HH} | Current hour (2 digits) |
| :{mm} | Current minutes (2 digits) |

## TODOs

- allow integration as Windows service.

# Schedulite

A simple scheduler for *lite* operations.

## Configuration

The list of tasks to run is configured via a json file.
While most fields are straightforward, the frequency is defined according to the UNIX cron format

Examples /  further reading:

- <https://www.ibm.com/docs/en/db2oc?topic=task-unix-cron-format#:~:text=The%20UNIX%20cron%20format%20is%20used%20to%20specify>,can%20be%20no%20blank%20within%20a%20field%20value.
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

## TODOs

- config file change detection, reissue loading of config.
    - do this only once
    - allow for different handling of first load of application and just a regular reload after the app is running
- feature: run task now if time to next execution greater than X.
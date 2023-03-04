# Schedulite

A simple scheduler for *lite* operations.

## Configuration

The list of tasks to run is configured via a json file.
While most fields are straightforward, the frequency is defined according to the UNIX cron format

Examples /  further reading:

- <https://www.ibm.com/docs/en/db2oc?topic=task-unix-cron-format#:~:text=The%20UNIX%20cron%20format%20is%20used%20to%20specify>,can%20be%20no%20blank%20within%20a%20field%20value.
- <https://docs.rs/job_scheduler/latest/job_scheduler/>
- <https://crates.io/crates/job_scheduler_ng>

#[path = "logger/logger.rs"]
mod logger;
#[path = "process/process.rs"]
mod process;
#[path = "runner/runner.rs"]
mod runner;

use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

use log::{debug, error, info, warn};
use sysinfo::System;

use crate::process::find_process;

const MAX_ATTEMPTS: u32 = 10;
const RETRY_DELAY: Duration = Duration::from_secs(1);
const SLEEP_DURATION_5: Duration = Duration::from_secs(5);

fn main() -> ExitCode {
    logger::init();

    info!("Running embedded dumper");
    match runner::run_embedded_dumper() {
        Ok(status) if status.success() => info!("Dumper finished successfully"),
        Ok(status) => {
            match status.code() {
                Some(code) => error!("Dumper exited with code {code}"),
                None => error!("Dumper was terminated before exiting"),
            }
            sleep(SLEEP_DURATION_5);
            return ExitCode::FAILURE;
        }
        Err(err) => {
            error!("Could not run the dumper: {err}");
            sleep(SLEEP_DURATION_5);
            return ExitCode::FAILURE;
        }
    }

    let process = "Notepad.exe";
    info!("Looking for process '{process}'");

    // Created outside the loop: find_process refreshes it on every call.
    let mut system = System::new();

    for attempt in 1..=MAX_ATTEMPTS {
        debug!("Refreshing process list (attempt {attempt}/{MAX_ATTEMPTS})");

        if let Some((pid, name)) = find_process(&mut system, process) {
            info!("Found '{name}' with PID {pid}");
            sleep(SLEEP_DURATION_5);
            return ExitCode::SUCCESS;
        }

        warn!("Not found, attempt {attempt}/{MAX_ATTEMPTS}");

        if attempt < MAX_ATTEMPTS {
            sleep(RETRY_DELAY);
        }
    }

    error!("Giving up: '{process}' not found after {MAX_ATTEMPTS} attempts");
    sleep(SLEEP_DURATION_5);
    ExitCode::FAILURE
}

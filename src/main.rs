#[path = "logger/logger.rs"]
mod logger;
#[path = "process/process.rs"]
mod process;

use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

use log::{debug, error, info, warn};
use sysinfo::System;

use crate::process::get_pid;

const MAX_ATTEMPTS: u32 = 10;
const RETRY_DELAY: Duration = Duration::from_secs(1);
const SLEEP_DURATION_5: Duration = Duration::from_secs(5);

fn main() -> ExitCode {
    logger::init();

    let process = "test.exe";
    info!("Looking for process '{process}'");

    // Reused across attempts: get_pid refreshes it on every call.
    let mut system = System::new();

    for attempt in 1..=MAX_ATTEMPTS {
        debug!("Refreshing process list (attempt {attempt}/{MAX_ATTEMPTS})");

        if let Some(pid) = get_pid(&mut system, process) {
            info!("Found '{process}' with PID {pid}");
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

use std::ffi::OsStr;

use sysinfo::{ProcessesToUpdate, System};

pub fn get_pid(system: &mut System, process: &str) -> Option<u32> {
    // `true` drops processes that exited since the last refresh, so we never
    // match a stale entry.
    system.refresh_processes(ProcessesToUpdate::All, true);

    system
        .processes_by_name(OsStr::new(process))
        .next()
        .map(|proc| proc.pid().as_u32())
}

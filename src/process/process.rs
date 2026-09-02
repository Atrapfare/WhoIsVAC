use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

pub fn find_process(system: &mut System, process: &str) -> Option<(u32, String)> {
    system.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());
    system.processes().values().find_map(|proc| {
        let name = proc.name().to_string_lossy();
        name.eq_ignore_ascii_case(process)
            .then(|| (proc.pid().as_u32(), name.into_owned()))
    })
}

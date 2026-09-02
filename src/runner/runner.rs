use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::{fs, io, thread, time::Duration};

const DUMPER_BYTES: &[u8] = include_bytes!("../../offsets/cs2-dumper.exe");

const DUMPER_NAME: &str = "cs2-dumper.exe";
const APP_DIR: &str = "WhoIsVAC";
const SUBDIR: &str = "offsets";

pub fn run_embedded_dumper() -> io::Result<ExitStatus> {
    let dir = target_dir()?;
    fs::create_dir_all(&dir)?;

    let exe = dir.join(DUMPER_NAME);
    fs::write(&exe, DUMPER_BYTES)?;

    let result = Command::new(&exe).current_dir(&dir).status();

    remove_best_effort(&exe);

    result
}

fn target_dir() -> io::Result<PathBuf> {
    let base = std::env::var_os("LOCALAPPDATA")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "LOCALAPPDATA is not set"))?;
    Ok(PathBuf::from(base).join(APP_DIR).join(SUBDIR))
}

// Retry briefly: on Windows an antivirus scan can hold a just-closed exe open.
fn remove_best_effort(path: &Path) {
    for _ in 0..3 {
        if fs::remove_file(path).is_ok() {
            return;
        }
        thread::sleep(Duration::from_millis(50));
    }
}

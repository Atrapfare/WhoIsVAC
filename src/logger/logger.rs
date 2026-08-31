use std::io::Write;

use env_logger::fmt::style::Style;
use env_logger::{Builder, Env, Target};
use jiff::Zoned;

/// Output is `HH:MM:SS LEVEL message` with a dimmed time and a coloured level.
/// The level is controlled by RUST_LOG (e.g. `RUST_LOG=debug`), info by default.
pub fn init() {
    let dim = Style::new().dimmed();

    Builder::from_env(Env::default().default_filter_or("info"))
        // env_logger defaults to stderr; the log is this program's actual
        // output, so `> log.txt` should capture it.
        .target(Target::Stdout)
        .format(move |buf, record| {
            let level = record.level();
            let level_style = buf.default_level_style(level);
            let time = Zoned::now().strftime("%H:%M:%S");

            writeln!(
                buf,
                "{dim}{time}{dim:#} {level_style}{level:<5}{level_style:#} {message}",
                message = record.args()
            )
        })
        .init();
}

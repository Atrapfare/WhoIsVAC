use std::io::Write;

use env_logger::fmt::style::Style;
use env_logger::{Builder, Env, Target};
use jiff::Zoned;

pub fn init() {
    let dim = Style::new().dimmed();

    Builder::from_env(Env::default().default_filter_or("info"))
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

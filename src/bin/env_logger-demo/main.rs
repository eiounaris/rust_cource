// RUST_LOG=trace cargo run --bin env_logger

use std::io::Write;
use log::{Level, info, warn, error, debug, trace};
use ansi_term::Colour::{Red, Yellow, Blue, Purple, White};
use env_logger::{Builder, Target};

fn main() {
    let mut builder = Builder::from_default_env();
    builder
        .target(Target::Stdout)
        .format(|buf, record| {
            // 根据不同级别选择颜色
            let level_colored = match record.level() {
                Level::Error => Red.bold().paint("ERROR"),
                Level::Warn  => Yellow.bold().paint("WARN"),
                Level::Info  => Blue.bold().paint("INFO"),
                Level::Debug => Purple.paint("DEBUG"),
                Level::Trace => White.paint("TRACE"),
            };
            writeln!(buf, "{}: {}", level_colored, record.args())
        });
    builder.init();

    error!("this is an error");
    warn!("this is a warning");
    info!("starting up");
    debug!("debug details");
    trace!("trace details");
}

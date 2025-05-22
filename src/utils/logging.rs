use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/// Initialize the logger with a custom format
pub fn init_logger() {
    let mut builder = Builder::new();
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info);

    // Check if we need to enable debug or trace logging
    if std::env::var("RUST_LOG").is_ok() {
        builder.parse_env("RUST_LOG");
    }

    builder.init();
}

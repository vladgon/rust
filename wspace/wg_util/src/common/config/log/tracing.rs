use crate::common::config::log::{get_log_level, Level};

pub fn init(default_level: Level) -> crate::Result<()> {
    Ok(tracing_subscriber::fmt()
        .with_target(true)
        .with_file(false)
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_line_number(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_max_level(get_log_level(default_level)?)
        .init())
}

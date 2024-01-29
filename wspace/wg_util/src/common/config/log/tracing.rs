use crate::common::config::log::Logger;

pub fn init(levels: &[Logger]) -> crate::Result<()> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_file(false)
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_line_number(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_env_filter(
            {
                let res = levels.iter()
                    .map(|log_entry| {
                        match log_entry {
                            Logger::LoggerForModule(module, level) => format!("{module}={}", level.to_string()),
                            Logger::LoggerRoot(level) => format!("{}", level.to_string()),
                        }
                    })
                    // .map(|log_entry: LogEntry| format!("{}{}", _0.as_ref().map(|v| format!("{v}=")).unwrap_or("".into()), _1.to_string()))
                    .collect::<Vec<String>>()
                    .join(",");
                println!("Setting Tracing filter config {res}");
                res
            }
        )
        .init();
    Ok(())
}


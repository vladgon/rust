use crate::common::config::log::Logger;
use crate::common::result_ext::ResultTap;
use crate::Result;

pub fn init(levels: &[Logger]) -> crate::Result<()> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_file(false)
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_line_number(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_env_filter(
            Result::Ok(levels.iter()
                             .map(|log_entry| {
                                 match log_entry {
                                     Logger::LoggerForModule(module, level) => format!("{module}={level}"),
                                     Logger::LoggerForModules(modules, level) =>
                                         modules.iter()
                                                .map(|module| format!("{module}={level}"))
                                                .collect::<Vec<_>>()
                                                .join(","),
                                     Logger::LoggerRoot(level) => format!("{level}"),
                                 }
                             })
                             .collect::<Vec<_>>()
                             .join(","))
                .tap(|res| println!("Setting Tracing filter config {res}"))
                .unwrap()
        )
        .init();
    Ok(())
}


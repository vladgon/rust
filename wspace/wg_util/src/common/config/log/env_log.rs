use log::debug;

use crate::common::config::log::{get_log_level, Level, Logger};
use crate::ResultExt;

pub fn init(log_level_entries: &[Logger]) -> crate::Result<()> {
    log_level_entries
        .iter()
        .fold(&mut env_logger::builder(),
              |b, l| {
                  match l {
                      Logger::LoggerForModule(module, level) => b.filter_module(module, get_log_level(level).unwrap().into()),
                      Logger::LoggerForModules(modules, level) =>
                          modules.iter()
                                 .fold(b, |b, module| b.filter_module(module, get_log_level(level).unwrap().into()))
                      ,
                      Logger::LoggerRoot(level) => b.filter(None, get_log_level(level).unwrap().into())
                  }
              })
        .try_init()
        .map(|_| debug!("Log initialized"))
        .into_std_error()
}

impl From<&Level> for tracing_subscriber::filter::LevelFilter {
    fn from(value: &Level) -> Self {
        match value {
            Level::Info => tracing_subscriber::filter::LevelFilter::INFO,
            Level::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
            Level::Error => tracing_subscriber::filter::LevelFilter::ERROR,
            Level::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
            Level::Off => tracing_subscriber::filter::LevelFilter::OFF,
        }
    }
}

impl From<&Level> for log::LevelFilter {
    fn from(value: &Level) -> Self {
        match value {
            Level::Info => log::LevelFilter::Info,
            Level::Debug => log::LevelFilter::Debug,
            Level::Error => log::LevelFilter::Error,
            Level::Trace => log::LevelFilter::Trace,
            Level::Off => log::LevelFilter::Off,
        }
    }
}

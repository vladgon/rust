use std::env;
use std::fmt::{Debug, Display, Formatter};

use crate::StdErrorBox;

pub const RUST_LOG: &str = "RUST_LOG";

mod tracing;
mod env_log;

pub enum Logger<'a> {
    LoggerRoot(Level),
    ///[&str] log module name, [Level] log level
    LoggerForModule(&'a str, Level),
    LoggerForModules(&'a [&'a str], Level),
}

pub struct LogConfig<'a> {
    kind: LogProvider,
    logger: &'a [Logger<'a>],
}


impl<'a> LogConfig<'a> {
    pub fn new(kind: LogProvider, logger: &'a [Logger]) -> Self { LogConfig { kind, logger } }
}

pub fn init(log_config: &LogConfig) -> crate::Result<()> {
    match log_config.kind {
        LogProvider::EnvLog => env_log::init(log_config.logger),
        LogProvider::Tracing => tracing::init(log_config.logger)
    }
}

impl Default for LogConfig<'_> {
    fn default() -> Self {
        LogConfig {
            kind: LogProvider::EnvLog,
            logger: &[Logger::LoggerRoot(Level::Debug)],
        }
    }
}

pub enum LogProvider {
    EnvLog,
    Tracing,
}

#[derive(Debug)]
pub enum Level {
    Info,
    Debug,
    Error,
    Trace,
    Off,
}

fn get_log_level(default_level: &Level) -> crate::Result<&Level> {
    match env::var(RUST_LOG) {
        Ok(env_level) => env_level.as_str().try_into(),
        Err(_) => Ok(default_level),
    }
}

impl TryFrom<&str> for &Level {
    type Error = StdErrorBox;

    fn try_from(value: &str) -> Result<Self, <&Level as TryFrom<&str>>::Error> {
        match value.to_lowercase().as_str() {
            "info" => Ok(&Level::Info),
            "debug" => Ok(&Level::Debug),
            "error" => Ok(&Level::Error),
            "trace" => Ok(&Level::Trace),
            "off" => Ok(&Level::Off),
            _ => { Err("Unsupported log level: {value}".into()) }
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_ascii_lowercase())
    }
}

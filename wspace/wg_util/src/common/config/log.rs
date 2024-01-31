use std::env;
use std::fmt::{Display, Formatter};

use anyhow::bail;

use crate::ResultExt;

pub const RUST_LOG: &str = "RUST_LOG";

mod tracing;
mod env_log;

pub enum Logger<'a> {
    LoggerRoot(Level),
    ///[&str] log module name, [Level]
    LoggerForModule(&'a str, Level),
}

pub struct LogConfig<'a> {
    kind: LogProvider,
    logger: &'a [Logger<'a>],
}


impl<'a> LogConfig<'a> {
    pub fn new(log_type: LogProvider, default_level: &'a [Logger]) -> Self {
        LogConfig { kind: log_type, logger: default_level }
    }
}

pub fn init(log_defaults: &LogConfig) -> crate::Result<()> {
    match log_defaults.kind {
        LogProvider::EnvLog => env_log::init(log_defaults.logger),
        LogProvider::Tracing => tracing::init(log_defaults.logger)
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
        .into_std_error()
}

impl TryFrom<&str> for &Level {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, <&Level as TryFrom<&str>>::Error> {
        match value.to_lowercase().as_str() {
            "info" => Ok(&Level::Info),
            "debug" => Ok(&Level::Debug),
            "error" => Ok(&Level::Error),
            "trace" => Ok(&Level::Trace),
            "off" => Ok(&Level::Off),
            _ => bail!("Unsupported log level: {value}")
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Info => write!(f, "info"),
            Level::Debug => write!(f, "debug"),
            Level::Error => write!(f, "error"),
            Level::Trace => write!(f, "trace"),
            Level::Off => write!(f, "off"),
        }
    }
}

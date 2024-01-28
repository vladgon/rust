use std::env;
use std::fmt::{Display, Formatter};

use anyhow::bail;
use log::LevelFilter;

use crate::ResultExt;

pub const RUST_LOG: &str = "RUST_LOG";

mod tracing;
mod env_log;


pub struct LogEntry<'a> {
    module: Option<&'a str>,
    level: Level,
}

impl<'a> LogEntry<'a> {
    pub fn new(module: &'a str, level: Level) -> Self {
        Self { module: Some(module), level }
    }
    pub fn all_modules(level: Level) -> Self {
        Self { module: None, level }
    }
}

pub struct LogDefaults<'a> {
    log_type: LogImplType,
    default_level: &'a [LogEntry<'a>],

}


impl<'a> LogDefaults<'a> {
    pub fn new(log_type: LogImplType, default_level: &'a [LogEntry]) -> Self {
        LogDefaults { log_type, default_level }
    }
}

impl Default for LogDefaults<'_> {
    fn default() -> Self {
        LogDefaults {
            log_type: LogImplType::EnvLog,
            default_level: &[LogEntry { module: None, level: Level::Debug }],
        }
    }
}

pub enum LogImplType {
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

pub fn init(log_defaults: LogDefaults) -> crate::Result<()> {
    match log_defaults.log_type {
        LogImplType::EnvLog => env_log::init(log_defaults.default_level),
        LogImplType::Tracing => tracing::init(log_defaults.default_level)
    }
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

impl From<&Level> for LevelFilter {
    fn from(value: &Level) -> Self {
        match value {
            Level::Info => LevelFilter::Info,
            Level::Debug => LevelFilter::Debug,
            Level::Error => LevelFilter::Error,
            Level::Trace => LevelFilter::Trace,
            Level::Off => LevelFilter::Off,
        }
    }
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

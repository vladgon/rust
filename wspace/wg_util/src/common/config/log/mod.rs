use std::env;

use anyhow::bail;
use log::LevelFilter::{Debug, Error, Info, Trace};

pub const RUST_LOG: &str = "RUST_LOG";

mod tracing;
mod env_log;

#[derive(Default)]
pub struct LogDefaults {
    pub log_type: LogImplType,
    pub default_level: Level,
}

#[derive(Default)]
pub enum LogImplType {
    #[default]
    EnvLog,
    Tracing,
}

#[derive(Default)]
pub enum Level {
    Info,
    #[default]
    Debug,
    Error,
    Trace,
}

pub fn init(log_defaults: LogDefaults) -> crate::Result<()> {
    match log_defaults.log_type {
        LogImplType::EnvLog => env_log::init(log_defaults.default_level),
        LogImplType::Tracing => tracing::init(log_defaults.default_level)
    }
}

fn get_log_level(default_level: Level) -> crate::Result<Level> {
    match env::var(RUST_LOG) {
        Ok(env_level) => env_level.try_into(),
        Err(_) => Ok(default_level),
    }.map_err(anyhow::Error::into)
}

impl TryFrom<String> for Level {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, <Level as TryFrom<String>>::Error> {
        match value.to_lowercase().as_str() {
            "info" => Ok(Level::Info),
            "debug" => Ok(Level::Debug),
            "error" => Ok(Level::Error),
            "trace" => Ok(Level::Trace),
            _ => bail!("Unsupported log level: {value}")
        }
    }
}

impl From<Level> for ::log::LevelFilter {
    fn from(value: Level) -> Self {
        match value {
            Level::Info => Info,
            Level::Debug => Debug,
            Level::Error => Error,
            Level::Trace => Trace,
        }
    }
}

impl From<Level> for tracing_subscriber::filter::LevelFilter {
    fn from(value: Level) -> Self {
        match value {
            Level::Info => tracing_subscriber::filter::LevelFilter::INFO,
            Level::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
            Level::Error => tracing_subscriber::filter::LevelFilter::ERROR,
            Level::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
        }
    }
}

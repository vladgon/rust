use std::env;

use anyhow::bail;
use anyhow::Result;
use log::{debug, LevelFilter, SetLoggerError};

pub const RUST_LOG: &str = "RUST_LOG";

pub fn init() -> Result<()> {
    env_logger::builder()
        .filter_level(get_log_level()?)
        .try_init()
        .map(|_| debug!("Log initialized"))
        .map_err(SetLoggerError::into)
}

fn get_log_level() -> Result<LevelFilter> {
    match env::var(RUST_LOG) {
        Ok(env_level) => LevelFilter::iter()
            .find(|s| s.as_str().eq_ignore_ascii_case(env_level.as_str()))
            .map(Ok)
            .unwrap_or_else(|| bail!("Log Level is invalid '{env_level}'")),
        Err(_) => Ok(LevelFilter::Info),
    }
}
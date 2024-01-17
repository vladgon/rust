use std::env;

use anyhow::Context;
use log::{debug, LevelFilter, SetLoggerError};

use crate::Result;

pub const RUST_LOG: &str = "RUST_LOG";

pub fn init(default_level: LevelFilter) -> Result<()> {
    env_logger::builder()
        .filter_level(get_log_level(default_level)?)
        .try_init()
        .map(|_| debug!("Log initialized"))
        .map_err(SetLoggerError::into)
}

fn get_log_level(default_level: LevelFilter) -> Result<LevelFilter> {
    let level_filter = match env::var(RUST_LOG) {
        Ok(env_level) => LevelFilter::iter()
            .find(|s| s.as_str().eq_ignore_ascii_case(env_level.as_str()))
            .with_context(|| "Log Level is invalid '{env_level}'")?,
        Err(_) => default_level,
    };
    Ok(level_filter)
}
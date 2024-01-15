use std::env;

use log::{debug, LevelFilter};

use crate::common::error::Errors;

pub fn init() -> Result<(), Errors> {
    env_logger::builder()
        .filter_level(get_log_level()?)
        .try_init()
        .map(|_| debug!("Log initialized"))
        .map_err(Errors::from)
}

fn get_log_level() -> Result<LevelFilter, Errors> {
    match env::var("RUST_LOG") {
        Ok(level) => LevelFilter::iter()
            .find(|s| s.as_str().eq_ignore_ascii_case(level.as_str()))
            .map(Ok)
            .unwrap_or_else(|| Err(Errors::Init(format!("Log Level is invalid '{level}'")))),
        Err(_) => Ok(LevelFilter::Info),
    }
}
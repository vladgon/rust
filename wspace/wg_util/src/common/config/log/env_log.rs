use log::{debug, SetLoggerError};

use crate::common::config::log::{get_log_level, Level};
use crate::Result;

pub fn init(default_level: Level) -> Result<()> {
    env_logger::builder()
        .filter_level(get_log_level(default_level)?.into())
        .try_init()
        .map(|_| debug!("Log initialized"))
        .map_err(SetLoggerError::into)
}

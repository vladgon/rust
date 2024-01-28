use log::debug;

use crate::{Result, ResultExt};
use crate::common::config::log::{get_log_level, LogEntry};

pub fn init(levels: &[LogEntry]) -> Result<()> {
    let builder = &mut env_logger::builder();
    levels.iter().fold(builder, |b, l| {
        b.filter(l.module, get_log_level(l.level).unwrap().into())
    })
        .try_init()
        .map(|_| debug!("Log initialized"))
        .into_std_error()
}

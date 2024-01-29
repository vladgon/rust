use log::debug;

use crate::{Result, ResultExt};
use crate::common::config::log::{get_log_level, LogLevelEntry};

pub fn init(log_level_entries: &[LogLevelEntry]) -> Result<()> {
    log_level_entries.iter()
        .fold(&mut env_logger::builder(),
              |b, l| {
                  match l {
                      LogLevelEntry::ModuleLevel(module, level) => b.filter_module(module, get_log_level(level).unwrap().into()),
                      LogLevelEntry::Level(level) => b.filter(None, get_log_level(level).unwrap().into())
                  }
              })
        .try_init()
        .map(|_| debug!("Log initialized"))
        .into_std_error()
}

use std::env::args;

use log::debug;

use crate::common;
use crate::common::config::app_config::{AppConfig, Init};
use crate::common::config::clap::AppConfigCLAP;
use crate::common::config::log::LogConfig;
use crate::Result;

pub fn init(log_defaults: LogConfig, use_clap: bool) -> Result<()> {
    common::config::log::init(log_defaults)?;

    // the first argument is 'inner:<command name>'
    let use_clap = use_clap || args().len() > 1;

    let args = if use_clap { AppConfigCLAP::init_clap() } else { AppConfigCLAP::default() };
    let files: Vec<&str> = args.config_files.split(',').collect();
    debug!("Using config files: {:?}", args.config_files);
    AppConfig::default().init_with_files(&files, args.env_override.unwrap())?;
    Ok(())
}
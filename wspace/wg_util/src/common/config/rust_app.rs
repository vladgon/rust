use log::debug;

use crate::common;
use crate::common::config::app_config::{AppConfig, ConfigInit};
use crate::common::config::clap::AppConfigCLAP;
use crate::common::config::log::LogDefaults;
use crate::Result;

pub fn init(log_defaults: LogDefaults, use_clap: bool) -> Result<()> {
    common::config::log::init(log_defaults)?;

    let args = if use_clap { AppConfigCLAP::init_clap() } else { AppConfigCLAP::default() };

    let files: Vec<&str> = args.config_files.as_str()
        .split(',')
        .collect();
    debug!("Using config files: {}", args.config_files);
    AppConfig::default().init_with_files(&files, args.env_override.unwrap())?;
    Ok(())
}
use log::{debug, LevelFilter};

use crate::common;
use crate::common::config::app_config::{AppConfig, ConfigInit};
use crate::common::config::clap::AppConfigCLAP;
use crate::Result;

pub fn init(default_level: LevelFilter, use_clap: bool) -> Result<()> {
    common::log::init(default_level)?;

    let args = if use_clap { AppConfigCLAP::init_clap() } else { AppConfigCLAP::default() };

    let files: Vec<&str> = args.config_files.as_str()
        .split(',')
        .collect();
    debug!("Using config files: {}", args.config_files);
    AppConfig::default().init_with_files(&files, args.env_override.unwrap())?;
    Ok(())
}
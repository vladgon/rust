use std::env;

use log::debug;

use crate::common;
use crate::common::config::app_config::{AppConfig, ConfigInit};
use crate::common::config::clap::AppConfigCLAP;
use crate::common::log::RUST_LOG;

pub fn init() -> anyhow::Result<()> {
    env::set_var(RUST_LOG, "DEBUG");
    env::set_var("db.user", "vlad2");
    common::log::init()?;

    let args = AppConfigCLAP::init();
    let files: Vec<&str> = args.config_files.as_str()
        .split(',')
        .collect();
    debug!("Using config files: {}", args.config_files);
    AppConfig::default().init_with_files(&files, args.env_override.unwrap())?;
    Ok(())
}
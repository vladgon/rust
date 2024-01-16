// extern crate wg_util;


use std::env;

use anyhow::Result;
use log::debug;

use wg_util::common;
use wg_util::common::config::app_config::{AppConfig, ConfigInit};
use wg_util::common::config::clap::AppConfigCLAP;
use wg_util::common::log::RUST_LOG;

fn main() -> Result<()> {
    env::set_var(RUST_LOG, "DEBUG");
    env::set_var("db.user", "vlad2");
    common::log::init()?;

    let args = AppConfigCLAP::init();
    let files: Vec<&str> = args.config_files.as_str()
        .split(',')
        .collect();
    debug!("Using config files: {}", args.config_files);
    AppConfig::default().init_with_files(&files, args.env_override.unwrap())?;

    anyhow::Ok(())
}

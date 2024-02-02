use std::env::args;

use anyhow::Context;
use log::debug;

use crate::common;
use crate::common::config::app_config::{AppConfig, Init};
use crate::common::config::clap::AppConfigCLAP;
use crate::common::config::log::LogConfig;
use crate::Result;

pub enum Options<'a> {
    Default,
    LogWithClap(LogConfig<'a>, bool),
}

pub fn init(options: Options) -> Result<()> {
    let (log_config, use_clap) = match options {
        Options::Default => (LogConfig::default(), false),
        Options::LogWithClap(log_config, use_clap) => (log_config, use_clap)
    };

    common::config::log::init(&log_config)?;

    // the first command line argument is 'inner:<command name>'
    let use_clap = use_clap || args().len() > 1;

    let args = if use_clap { AppConfigCLAP::init_clap() } else { AppConfigCLAP::default() };
    debug!("Using config files: {:?}", args.config_files);
    let files = args.config_files.split(',').collect::<Vec<_>>();
    AppConfig::default().init_with_files(&files, args.env_override.with_context(|| "cannot process env_override")?)?;
    Ok(())
}
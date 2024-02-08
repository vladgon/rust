use std::env::args;

use anyhow::Context;
use log::debug;
use regex::Regex;

use crate::common;
use crate::common::config::app_config::{AppConfig, Init};
use crate::common::config::clap::AppConfigCLAP;
use crate::common::config::log::LogConfig;
use crate::Result;

pub enum Options<'a> {
    DefaultLogNoClap,
    DefaultLog(bool),
    LogAndClap(LogConfig<'a>, bool),
}

pub fn init(options: Options) -> Result<()> {
    let (log_config, is_clap) = match options {
        Options::DefaultLogNoClap => (LogConfig::default(), false),
        Options::DefaultLog(is_clap) => (LogConfig::default(), is_clap),
        Options::LogAndClap(log_config, is_clap) => (log_config, is_clap)
    };

    common::config::log::init(&log_config)?;
    let clap_pattern = Regex::new(r"-c\s|--config_files\s")?;
    let is_clap = is_clap || args().any(|arg| clap_pattern.is_match(arg.as_str()));

    let args = if is_clap { AppConfigCLAP::init_clap() } else { AppConfigCLAP::default() };
    debug!("Using config files: {:?}", args.config_files);
    let files = args.config_files.split(',').collect::<Vec<_>>();
    AppConfig::default().init_with_files(&files, args.env_override.with_context(|| "cannot process env_override")?)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use regex::Regex;

    #[test]
    fn parsing() {
        let re = Regex::new(r"-c\s|--config_files\s").unwrap();

        let value = "abc -c filename --config_files long_file_name";
        println!("Matching {value}");
        assert!(re.is_match(value), "Should match {value}");

        let value = "abc -cfilename --config_files long_file_name";
        println!("Matching {value}");
        assert!(re.is_match(value), "Should match {value}");

        let value = "abc -cfilename  --config_filelong_file_name";
        assert!(!re.is_match(value), "Should not match {value}");
        println!("Not Matching {value}");


        let value = "abc -cfilename";
        println!("Not Matching {value}");
        assert!(!re.is_match(value), "Should not match {value}");
    }
}


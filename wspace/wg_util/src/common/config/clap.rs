use std::fs::metadata;
use std::path::Path;

use clap::Parser;
use log::debug;

use crate::common::io::cargo_work_space_home;
use crate::common::result_ext::ResultTap;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppConfigCLAP {
    /// wg_sample_app files list
    #[arg(short, long, value_name = "FILE")]
    pub config_files: String,

    /// environment override, default value true, possible values: true, false
    #[arg(short, long, value_names = ["true,false"], default_value = "true")]
    pub env_override: Option<bool>,
}

const CONFIG_FILE: &str = "resources/app_config.yaml";

impl AppConfigCLAP {
    ///Parses command line arguments
    pub fn init_clap() -> Self {
        Self::parse()
    }
    fn derive_path() -> String {
        let path = cargo_work_space_home()
            .map(|cargo_home| {
                let path = format!("wg_sample_app/{CONFIG_FILE}");
                let config_path = Path::new(path.as_str());
                Path::new(cargo_home.as_str()).join(config_path)
            })
            .tap_err(|e| debug!("{e}, trying {:?}", CONFIG_FILE))
            .unwrap_or_else(|_| Path::new(CONFIG_FILE).into());
        assert!(metadata(&path).unwrap().is_file(), "File should exists");
        path.to_str().unwrap().to_string()
    }
}

impl Default for AppConfigCLAP {
    fn default() -> Self {
        Self {
            config_files: Self::derive_path(),
            env_override: Some(true),
        }
    }
}
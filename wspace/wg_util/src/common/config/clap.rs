use std::fs::metadata;
use std::path::Path;

use clap::Parser;
use log::debug;

use crate::common::io::cargo_work_space_home;
use crate::common::result_ext::ResultTap;

/// Simple program to greet a person
#[derive(Parser)]
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
        cargo_work_space_home()
            .map(|ref cargo_home| Path::new(cargo_home).join(format!("wg_sample_app/{CONFIG_FILE}")))
            .tap_err(|e| debug!("trying {CONFIG_FILE}, {e}"))
            .tap(|path| assert!(metadata(path).unwrap().is_file(), "File should exists"))
            .unwrap_or_else(|_| CONFIG_FILE.into())
            .to_str()
            .unwrap()
            .to_string()
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
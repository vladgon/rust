use std::fs::metadata;
use std::path::Path;

use clap::Parser;

use crate::common::io::cargo_work_space_home;

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

impl AppConfigCLAP {
    ///Parses command line arguments
    pub fn init_clap() -> Self {
        Self::parse()
    }
    fn derive_path() -> String {
        let cargo_home = cargo_work_space_home().unwrap();
        let config_path = Path::new("wg_sample_app/resources/app_config.yaml");
        let path = &Path::new(cargo_home.as_str()).join(config_path);
        assert!(metadata(path).unwrap().is_file(), "File should exists");
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
use std::path::Path;

use clap::Parser;

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
    pub fn init_no_clap(config_files: String, env_override: Option<bool>) -> Self {
        Self {
            config_files,
            env_override,
        }
    }

    fn derive_path() -> String {
        let path1 = Path::new("wg_sample_app/resources/app_config.yaml");
        let path2 = Path::new("..").join(path1);
        if path1.exists() {
            path1.to_str().unwrap().to_string()
        } else  {
            path2.to_str().unwrap().to_string()
        }
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
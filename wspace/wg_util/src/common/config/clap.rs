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
    pub fn init() -> Self {
        Self::parse()
    }
}
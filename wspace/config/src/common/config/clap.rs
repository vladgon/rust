use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MyArgs {
    /// config files list
    #[arg(short, long, value_name = "FILE")]
    pub config_files: String,

    /// environment override, default value true, possible values: true, false
    #[arg(short, long,  value_names = ["true,false"], default_value="true")]
    pub env_override: Option<bool>,
}

impl MyArgs {
    pub fn init() -> Self {
        Self::parse()
    }
}


#[test]
fn verify_cli() {
    use clap::CommandFactory;
    MyArgs::command().debug_assert()
}
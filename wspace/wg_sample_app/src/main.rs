extern crate wg_util;

use std::env;

use wg_util::common::config::log::LogConfig;
use wg_util::common::config::rust_app;
use wg_util::common::config::rust_app::Options::LogWithClap;
use wg_util::Result;

fn main() -> Result<()> {
    env::set_var("db.user", "vlad2");
    rust_app::init(LogWithClap(LogConfig::default(), true))
}

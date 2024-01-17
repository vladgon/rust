extern crate wg_util;

use std::env;

use log::LevelFilter::Debug;

use wg_util::common::config::rust_app;
use wg_util::Result;

fn main() -> Result<()> {
    env::set_var("db.user", "vlad2");
    rust_app::init(Debug, true)
}

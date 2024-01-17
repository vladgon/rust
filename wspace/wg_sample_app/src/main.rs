extern crate wg_util;

use anyhow::Result;

use wg_util::common::config::rust_app;

fn main() -> Result<()> {
    rust_app::init()
}

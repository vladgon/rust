use std::env;
use std::env::current_dir;
use std::path::Path;

use anyhow::{Context, Result};

use wg_util::common::io::copy_recursively;

const RESOURCE_DIR: &str = "resources";

const OUT_DIR: &str = "OUT_DIR";

macro_rules! cargo_info {
    ($($tokens: tt)*) => {
        println!("cargo:warning=\r\x1b[32;1m   {}", format!($($tokens)*))
    }
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={RESOURCE_DIR}");
    cargo_info!("build.rs: Copying Resources");
    let resource_path = current_dir().map(|path| path.join(RESOURCE_DIR))?;

    let target = env::var(OUT_DIR)?;

    let dest_path = Path::new(target.as_str())
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .with_context(|| "Cannot get parent path")?
        .join(RESOURCE_DIR);

    cargo_info!("Copying from {:?}: to {:?}", resource_path, dest_path);
    copy_recursively(resource_path, dest_path)
}

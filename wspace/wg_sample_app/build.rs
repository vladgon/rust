use std::env;
use std::env::current_dir;
use std::path::Path;

use anyhow::{Context, Result};

use wg_util::common::io::copy_recursively;

const RESOURCE_DIR: &str = "resources";

const OUT_DIR: &str = "OUT_DIR";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={RESOURCE_DIR}");
    println("build.rs: Copying Resources");
    let resource_path = current_dir().map(|path| path.join(RESOURCE_DIR))?;

    let target = env::var(OUT_DIR)?;

    let dest_path = Path::new(target.as_str())
        .parent()
        .map(Path::parent)
        .flatten()
        .map(Path::parent)
        .flatten()
        .with_context(|| "Cannot get parent path")?
        .join(RESOURCE_DIR);

    println(format!("Copying from {:?}: to {:?}", resource_path, dest_path).as_str());
    copy_recursively(resource_path, dest_path)
}

fn println(value: &str) { println!("cargo:warning={value}"); }

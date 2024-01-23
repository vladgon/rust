use std::env;
use std::env::current_dir;
use std::path::Path;

use anyhow::Context;

use wg_util::{info_build, Result};
use wg_util::common::build_util::{OUT_DIR, RESOURCE_DIR};
use wg_util::common::io::copy_recursively;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={RESOURCE_DIR}");
    info_build!("build.rs: Copying Resources");
    let resource_path = current_dir().map(|path| path.join(RESOURCE_DIR))?;

    let target = env::var(OUT_DIR)?;

    let dest_path = Path::new(target.as_str())
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .with_context(|| "Cannot get parent path")?
        .join(RESOURCE_DIR);

    info_build!("Copying from {:?}: to {:?}", resource_path, dest_path);
    copy_recursively(resource_path, dest_path)
}

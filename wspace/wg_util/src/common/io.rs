use std::{env, fs};
use std::path::Path;

use anyhow::Context;

use crate::{Result, ResultExt};

const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
const CARGO_PKG_NAME: &str = "CARGO_PKG_NAME";

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&destination)?;
    fs::read_dir(source)
        .map(|mut read_dir| read_dir
            .try_for_each(|entry| {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))
                } else {
                    _ = fs::copy(entry.path(), destination.as_ref().join(entry.file_name()));
                    Ok(())
                }
            }))?
}

pub fn cargo_work_space_home() -> Result<String> {
    let manifest = env::var(CARGO_MANIFEST_DIR)
        .with_context(|| format!("Cannot read env var {CARGO_MANIFEST_DIR}"))?;
    let pkg_name = env::var(CARGO_PKG_NAME)
        .with_context(|| format!("Cannot read env var {CARGO_PKG_NAME}"))?;

    manifest.as_str().strip_suffix(pkg_name.as_str())
        .map(|s| s.to_string())
        .with_context(|| format!("Error getting cargo home ManifestDir {manifest}, pkgName: {pkg_name}"))
        .into_std_error()
}

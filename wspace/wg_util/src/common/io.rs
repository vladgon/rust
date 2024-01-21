use std::{env, fs};
use std::path::Path;

use anyhow::Context;

use crate::{Result, ResultExt};

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&destination)?;
    fs::read_dir(source)?
        .try_for_each(|entry| {
            let entry = entry?;
            let filetype = entry.file_type()?;
            if filetype.is_dir() {
                copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))
            } else {
                _ = fs::copy(entry.path(), destination.as_ref().join(entry.file_name()));
                Ok(())
            }
        })
}

pub fn cargo_work_space_home() -> Result<String> {
    let manifest = env::var("CARGO_MANIFEST_DIR")?;
    let pkg_name = env::var("CARGO_PKG_NAME")?;

    manifest.as_str().strip_suffix(pkg_name.as_str())
        .map(|s| s.to_string())
        .with_context(|| format!("Error getting cargo home ManifestDir {manifest}, pkgName: {pkg_name}"))
        .into_std_error()
}

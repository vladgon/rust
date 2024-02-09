use std::{env, fs};
use std::path::Path;

use anyhow::anyhow;
use log::debug;

use crate::{Result, ResultExt};
use crate::common::result_ext::ResultTap;

const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
const CARGO_PKG_NAME: &str = "CARGO_PKG_NAME";

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&destination)?;
    fs::read_dir(source)?
        .try_for_each(|entry|
            entry.map(|entry|
                if entry.file_type()?.is_dir() {
                    copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))
                } else {
                    _ = fs::copy(entry.path(), destination.as_ref().join(entry.file_name()));
                    Ok(())
                })?
        )
}

pub fn cargo_work_space_home() -> Result<String> {
    env::var(CARGO_MANIFEST_DIR)
        .tap_err(|e| debug!("{e}"))
        .map(|manifest| {
            env::var(CARGO_PKG_NAME)
                .tap_err(|e| debug!("{e}"))
                .map(|pkg_name| {
                    manifest.as_str().strip_suffix(pkg_name.as_str())
                            .map(|s| s.to_string())
                            .ok_or(anyhow!("Error getting cargo home ManifestDir {manifest}, pkgName: {pkg_name}"))
                })?
        })?
        .into_std_error()
}

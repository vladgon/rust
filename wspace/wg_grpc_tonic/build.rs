use std::fs;
use std::path::{Path, PathBuf};

use wg_util::{info_build, Result, ResultExt};

const PROTO_ROOT: &str = "proto";

fn main() -> Result<()> {
    tonic_build::configure()
        .include_file("include.rs")
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile(files(PROTO_ROOT)?.as_slice(), &[PROTO_ROOT])
        .into_std_error()
}

fn files<T: AsRef<Path>>(path: T) -> Result<Vec<PathBuf>> {
    info_build!("Processing {:?}", path.as_ref());
    fs::read_dir(&path)
        .map(|dir| dir.map(|d| {
            let path = d.unwrap_or_else(|e| panic!("{}", e)).path();
            info_build!("Processing {}", path.to_str().unwrap());
            path
        }).collect::<Vec<PathBuf>>())
        .into_std_error()
}


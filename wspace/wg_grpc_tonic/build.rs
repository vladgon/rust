use std::{env, fs};
use std::path::{Path, PathBuf};

use prost_wkt_build::{FileDescriptorSet, Message};

use wg_util::{info_build, Result, ResultExt};

const PROTO_ROOT: &str = "proto";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={PROTO_ROOT}");
    let out = PathBuf::from(env::var("OUT_DIR")?);
    let descriptor_file = out.join("descriptors.bin");
    tonic_build::configure()
        .include_file({
            info_build!("Generating include file: {:?}",out.join("include.rs"));
            "include.rs"
        })
        .type_attribute(
            ".",
            "#[derive(serde::Serialize,serde::Deserialize)]",
        )
        .extern_path(
            ".google.protobuf.Any",
            "::prost_wkt_types::Any",
        )
        .extern_path(
            ".google.protobuf.Timestamp",
            "::prost_wkt_types::Timestamp",
        )
        .extern_path(
            ".google.protobuf.Value",
            "::prost_wkt_types::Value",
        )
        .file_descriptor_set_path(&descriptor_file)
        .compile(files(PROTO_ROOT)?.as_slice(), &[PROTO_ROOT])?;

    info_build!("Adding WKT descriptor_file: {:?}",descriptor_file);

    let descriptor_bytes = fs::read(descriptor_file)?;
    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..])?;
    prost_wkt_build::add_serde(out, descriptor);
    Ok(())
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

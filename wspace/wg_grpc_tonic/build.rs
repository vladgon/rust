use std::{env, fs};
use std::fs::DirEntry;
use std::io::Error;
use std::path::{Path, PathBuf};

use prost_wkt_build::{FileDescriptorSet, Message};

use wg_util::{info_build, ResultTap};

const PROTO_ROOT: &str = "proto";

const INCLUDE_FILE: &str = "include.rs";

fn main() -> wg_util::Result<()> {
    println!("cargo:rerun-if-changed={PROTO_ROOT}");
    let out = PathBuf::from(env::var("OUT_DIR")?);
    let descriptor_file = out.join("descriptors.bin");
    tonic_build::configure()
        .include_file({
            info_build!("Using include file: {:?}",out.join(INCLUDE_FILE));
            INCLUDE_FILE
        }
        )
        .type_attribute(".",
                        "#[derive(serde::Serialize,serde::Deserialize)]")
        .extern_path(".google.protobuf.Any",
                     "::prost_wkt_types::Any")
        .extern_path(".google.protobuf.Timestamp",
                     "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Value",
                     "::prost_wkt_types::Value")
        .file_descriptor_set_path(&descriptor_file)
        .compile(files(PROTO_ROOT)?.as_slice(), &[PROTO_ROOT])?;

    info_build!("Adding WKT descriptor_file: {:?}",descriptor_file);

    let descriptor_bytes = fs::read(descriptor_file)?;
    let descriptor = FileDescriptorSet::decode(descriptor_bytes.as_slice())?;
    prost_wkt_build::add_serde(out, descriptor);
    Ok(())
}

fn files(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, Error> {
    info_build!("Processing root dir {:?}", path.as_ref());
    fs::read_dir(&path)
        .map(|read_dir| read_dir
            .map(|dir_entry| dir_entry.as_ref()
                                      .map(DirEntry::path)
                                      .tap_ok(|path| info_build!("Processing file {:?}", path))
                                      .unwrap())
            .collect::<Vec<PathBuf>>()
        )
}

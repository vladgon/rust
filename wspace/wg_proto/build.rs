use std::env;
use std::path::PathBuf;

use prost_build::Config;
use prost_wkt_build::{FileDescriptorSet, Message};
use wg_util::info_build;

const PROTO_ROOT: &str = "proto";
const PROTO_PACKAGE: &str = "proto.model";

static FILE_NAMES: &[&str] = &[
    "sample",
    "sample1"
];
/// Custom name replace tuple (\<JSON Path\>, \<New Name\>)
static JSON_NAMES: &[(&str, &str)] = &[
    ("Shirt.color", "color_custom")
];

fn main() -> wg_util::Result<()> {
    env::set_var("CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG", "true");
    println!("cargo:rerun-if-changed=proto");
    let config = &mut Config::new();
    let config = JSON_NAMES
        .iter()
        .fold(config,
              |config, &t2|
                  config.field_attribute(t2.0,
                                         format!(r##"#[serde(rename(serialize = "{}", deserialize = "{}"))]"##,
                                                 t2.1,
                                                 t2.1
                                         )));
    _ = wkt(config)?;
    Ok(())
}

fn wkt(config: &mut Config) -> wg_util::Result<&mut Config> {
    let out = PathBuf::from(env::var("OUT_DIR")?);
    let descriptor_file = out.join("descriptors.bin");
    config
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
        .include_file({
            info_build!("Generating include file: {}/{PROTO_PACKAGE}.include.rs",out.to_str().unwrap());
            format!("{PROTO_PACKAGE}.include.rs")
        })
        .compile_protos(FILE_NAMES.iter()
                            .map(|name| format!("{PROTO_ROOT}/{name}.proto"))
                            .collect::<Vec<_>>()
                            .as_ref(),
                        &[format!("{}/", PROTO_PACKAGE)])?;
    info_build!("Generating WKT descriptor_file:  {:?}",descriptor_file);


    let descriptor_bytes = std::fs::read(descriptor_file)?;

    let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..])?;

    prost_wkt_build::add_serde(out, descriptor);
    Ok(config)
}

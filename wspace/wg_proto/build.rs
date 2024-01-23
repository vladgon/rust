use std::env;

const PROTO_ROOT: &str = "proto";
const PROTO_PACKAGE: &str = "wg.proto.model";

static SERDE_NAMES: &[&str] = &[
    "Shirt",
    "Foo1"
];

static FILE_NAMES: &[&str] = &[
    "sample",
    "sample1"
];

fn main() -> std::io::Result<()> {
    env::set_var("ARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG", "true");
    println!("cargo:rerun-if-changed=proto");
    let config = &mut prost_build::Config::new();
    SERDE_NAMES
        .iter()
        .fold(config,
              |config, &name|
                  config.type_attribute(with_pkg(name), "#[derive(serde::Serialize, serde::Deserialize)]")
                      .type_attribute(with_pkg(name), "#[serde(default)]"),
        )
        .compile_protos(FILE_NAMES.iter()
                            .map(|name| format!("{PROTO_ROOT}/{name}.proto"))
                            .collect::<Vec<String>>()
                            .as_ref(),
                        &[format!("{}/", PROTO_PACKAGE)])
}

fn with_pkg(name: &str) -> String { format!("{PROTO_PACKAGE}.{name}") }
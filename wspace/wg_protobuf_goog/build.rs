use std::fs;
use std::path::{Path, PathBuf};

use wg_util::{info_build, Result, ResultExt};

const PROTO_ROOT: &str = "src/proto";
const GENERATED_OUT_DIR: &str = "proto_generated";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={}", PROTO_ROOT);

    let code_gen = &mut protobuf_codegen::Codegen::new();
    files(PROTO_ROOT)?
        .iter()
        .fold(code_gen,
              |codegen, file| codegen.input(file))
        .protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        .protoc_path(&protoc_bin_vendored::protoc_bin_path()?)
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes([PROTO_ROOT])
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir(GENERATED_OUT_DIR)
        .run()
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
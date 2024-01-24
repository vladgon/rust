use wg_util::Result;

const PROTO_ROOT: &str = "src/proto";
const GENERATED_OUT_DIR: &str = "proto_generated";

const FILES: &[&str] = &[
    "sample.proto",
    "sample1.proto"
];

fn main() -> Result<()> {
    let code_gen = &mut protobuf_codegen::Codegen::new();
    let code_gen = FILES.iter()
        .fold(code_gen, |codegen, file| codegen.input(format!("{}/{}", PROTO_ROOT, file)));
    // Use `protoc` parser, optional.
    code_gen.protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&[PROTO_ROOT])
        // Inputs must reside in some of include paths.
        // .input("src/proto/sample.proto")
        // .input("src/proto/sample1.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir(GENERATED_OUT_DIR)
        .run_from_script();
    Ok(())
}

use minifier::js::minify;
use std::env;
use std::fs;
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use std::string::String;

fn main() -> Result<(), std::io::Error> {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("OutDir {}", out_dir);
    let src_path = Path::new(&out_dir).parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .and_then(|p| Some(p.join("webui.js"))).unwrap();


//        .unwrap().join("webui.jcas");

    println!("Src Path {:?}", src_path);
    let dest_path = Path::new(&out_dir).join("../../../webui.min.js");
    let result = fs::read_to_string(src_path);
    match result {
        Ok(content) => {
            println!("webui.js exists {}", content);
            fs::write(dest_path, minify(content.as_str()).as_bytes());
        }
        Err(err) => println!("webui.js missing {:?}", err)
    }

    Ok(())
}
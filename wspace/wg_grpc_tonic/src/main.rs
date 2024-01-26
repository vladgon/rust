tonic::include_proto!("helloworld");

pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), "/helloworld.rs"));
}

fn main() {
    println!("Hello, world!");
}

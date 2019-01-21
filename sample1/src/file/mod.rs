pub mod inside_file;
pub mod inside;

pub fn open_file() {
    inside::inside_file1::yy();
    println!("Open file");
}
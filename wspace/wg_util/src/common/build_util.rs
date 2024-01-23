pub const RESOURCE_DIR: &str = "resources";

pub const OUT_DIR: &str = "OUT_DIR";

#[macro_export]
macro_rules! info_build {
    ($($tokens: tt)*) => {
        println!("cargo:warning=\r\x1b[32;1m   {}", format!($($tokens)*))
       }
}

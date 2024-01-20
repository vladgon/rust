pub mod common;

pub type StdErrorBox = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, StdErrorBox>;

pub trait ResultExt<T> {
    fn into_std_error(self) -> Result<T>;
}

impl<T, E: Into<StdErrorBox>> ResultExt<T> for std::result::Result<T, E> {
    fn into_std_error(self) -> Result<T> { self.map_err(|e| e.into()) }
}

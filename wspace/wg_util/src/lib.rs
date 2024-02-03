pub use common::option_ext::OptionTap;
pub use common::result_ext::{Result, ResultExt, ResultTap};

pub mod common;

pub type StdErrorBox = Box<dyn std::error::Error>;


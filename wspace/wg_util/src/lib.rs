pub use common::iterator_ext::IteratorExt;
pub use common::option_ext::OptionExt;
pub use common::result_ext::{Result, ResultExt};

pub mod common;

pub type StdErrorBox = Box<dyn std::error::Error>;



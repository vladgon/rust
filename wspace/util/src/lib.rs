///
///`Result` type with void and generic Error, mainly to support `?`shortcut
///
pub type ResultOK = Result<(), Box<std::error::Error>>;

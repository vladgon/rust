///
///`Result` type with void and generic [`Error`], mainly to support `?`shortcut
///
///
/// [`Error`]: std::error::Error
///
pub type ResultOK = Result<(), dyn std::error::Error>;
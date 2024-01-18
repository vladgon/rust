use std::fmt;

#[allow(dead_code)]
pub enum LogLevel {
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Warn => f.write_str("warning"),
            LogLevel::Error => f.write_str("common"),
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_log {
    ($level:expr, $msg:expr) => {
        println!("{}: {}", $level, $msg)
    }
}
#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
        __impl_log!($crate::log::LogLevel::Warn, format_args!($($args)*))
    }
}

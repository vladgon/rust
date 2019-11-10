use std::fmt;

//#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum LogLevel {
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Warn => write!(f, "warning"),
            LogLevel::Error => write!(f, "error"),
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

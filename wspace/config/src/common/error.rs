use std::error::Error;
use std::fmt::{Display, Formatter};

use config::ConfigError;
use log::SetLoggerError;

#[derive(Debug)]
pub(crate) enum Errors {
    Init(String),
    Parse(String),
    AnyHow(String),
}

impl Error for Errors {}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::Init(s) | Errors::Parse(s) | Errors::AnyHow(s) => write!(f, "{s}"),
        }
    }
}

impl From<ConfigError> for Errors {
    fn from(value: ConfigError) -> Self {
        Errors::Init(value.to_string())
    }
}

impl From<SetLoggerError> for Errors {
    fn from(value: SetLoggerError) -> Self {
        Errors::Init(value.to_string())
    }
}

impl From<anyhow::Error> for Errors {
    fn from(value: anyhow::Error) -> Self {
        Errors::AnyHow(value.to_string())
    }
}
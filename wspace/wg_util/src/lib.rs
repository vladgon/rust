use std::error::Error;

pub mod common;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
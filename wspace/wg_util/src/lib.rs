use std::error::Error;

pub mod common;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
// pub type Result<T> = std::result::Result<T, Error>;

// #[derive(Error, Debug)]
// enum MyError {
//     // #[error("Set Logger Error")]
//     // setloggere(SetLoggerError),
//
//     #[error(transparent)]
//     SetLoggerError(#[from]  SetLoggerError),
//     // #[error(transparent)]
//     // AnyHowError(#[from]  anyhow::Error),
//     // #[error(transparent)]
//     // SerdeDesStdError(#[from]  Box<dyn StdError>),
//     // #[error(transparent)]
//     // IOError(#[from]  std::io::Error),
//     //
// }
use std::str::FromStr;

use tracing::Level;

use crate::START;

const MAX_LEVEL: Level = Level::DEBUG;

pub fn init() {
    START.call_once(|| {
        let level = std::env::var("RUST_LOG")
            .or_else(|_| {
                println!("env var 'RUST_LOG' is not defined");
                Ok(MAX_LEVEL.to_string())
            })
            .and_then(|level| Level::from_str(level.as_str()))
            .unwrap();

        println!("Setting Log Level '{}'", level.to_string());
        tracing_subscriber::fmt::fmt()
            .with_max_level(level)
            .init();
    })
}

pub fn db_url() -> Result<String, dotenv::Error> {
    init();
    dotenv::var("DB_URL")
}

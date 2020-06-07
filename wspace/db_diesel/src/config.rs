use std::error::Error;
use std::str::FromStr;
use std::sync::Once;

use tracing::Level;

static START: Once = Once::new();

pub fn init() {
    const MAX_LEVEL: Level = Level::DEBUG;
    const RUST_LOG: &str = "RUST_LOG";
    START.call_once(|| {
        let level: Result<Level, Box<dyn Error>> = std::env::var(RUST_LOG)
            .map_err(|e| e.into())
            .and_then(|level| Level::from_str(level.as_str()).map_err(|e| e.into()));

        let level = level.unwrap_or_else(|e| {
            println!("RUST_LOG: {}", e);
            MAX_LEVEL
        });

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

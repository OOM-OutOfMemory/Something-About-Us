use anyhow::{anyhow, Result};
use tracing::Level;

use crate::infrastructure::config::types::LoggerConfig;

pub fn init_logger(config: &LoggerConfig) -> Result<()> {
    let level = match config.level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => {
            return Err(anyhow!("unsupported log level: {}", config.level.as_str()));
        }
    };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .init();
    Ok(())
}

use anyhow::{Ok, Result};

use crate::infrastructure::config::types::Config;

pub fn check_config_validation(config: Config) -> Result<Config> {
    Ok(config)
}

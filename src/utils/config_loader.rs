use crate::config::Config;
use crate::error::{JigError, Result};
use std::fs;
use std::path::Path;

pub fn load_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
    let content = fs::read_to_string(path)?;
    Config::from_yaml(&content)
}

pub fn get_default_config_path() -> Result<std::path::PathBuf> {
    let home = std::env::var("HOME")
        .map_err(|_| JigError::Config("HOME environment variable not set".to_string()))?;
    Ok(std::path::PathBuf::from(home).join(".jig.yaml"))
}

pub fn load_or_create_config() -> Result<Config> {
    let config_path = get_default_config_path()?;

    if config_path.exists() {
        load_config_from_file(config_path)
    } else {
        // Return default config
        Ok(Config::new())
    }
}

use anyhow::{Context, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration for the jig tool
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Path to the baseline repository
    pub baseline_path: PathBuf,

    /// Configuration for templates
    pub templates: TemplatesConfig,

    /// Configuration for AI tools
    pub ai: AiConfig,
}

/// Configuration for templates
#[derive(Debug, Serialize, Deserialize)]
pub struct TemplatesConfig {
    /// Path to the templates directory
    pub templates_dir: PathBuf,

    /// Additional template variables to include
    #[serde(default)]
    pub variables: std::collections::HashMap<String, String>,
}

/// Configuration for AI tools
#[derive(Debug, Serialize, Deserialize)]
pub struct AiConfig {
    /// Path to the baseline AI configuration files
    pub config_dir: PathBuf,

    /// Tools to configure
    #[serde(default)]
    pub tools: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let baseline_path = PathBuf::from("/Users/dave/src/github.com/daveio/_baseline");

        Config {
            baseline_path: baseline_path.clone(),
            templates: TemplatesConfig {
                templates_dir: PathBuf::from("templates"),
                variables: std::collections::HashMap::new(),
            },
            ai: AiConfig {
                config_dir: baseline_path,
                tools: vec![
                    "claude-desktop".to_string(),
                    "cursor".to_string(),
                    "zed".to_string(),
                    "goose".to_string(),
                ],
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;

        if !config_path.exists() {
            debug!("Config file not found, using default configuration");
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path).context(format!(
            "Failed to read config file at {}",
            config_path.display()
        ))?;

        let config: Config =
            toml::from_str(&content).context("Failed to parse configuration file")?;

        debug!("Loaded configuration from {}", config_path.display());

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;

        // Create parent directories if needed
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        fs::write(&config_path, content).context(format!(
            "Failed to write config to {}",
            config_path.display()
        ))?;

        debug!("Saved configuration to {}", config_path.display());

        Ok(())
    }
}

/// Get the path to the configuration file
fn get_config_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Failed to get home directory")?;

    Ok(home_dir.join(".config/jig/config.toml"))
}

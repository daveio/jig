use anyhow::{anyhow, Context, Result};
use config::{Config, Environment, File};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use yaml_rust2::{YamlEmitter, YamlLoader};

/// Configuration for the jig tool
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    /// Path to the baseline repository
    #[serde(default = "default_baseline_path")]
    pub baseline_path: PathBuf,

    /// Configuration for templates
    #[serde(default)]
    pub templates: TemplatesConfig,

    /// Configuration for AI tools
    #[serde(default)]
    pub ai: AiConfig,
}

/// Configuration for templates
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TemplatesConfig {
    /// Path to the templates directory
    #[serde(default = "default_templates_dir")]
    pub templates_dir: PathBuf,

    /// Additional template variables to include
    #[serde(default)]
    pub variables: HashMap<String, String>,
}

/// Configuration for AI tools
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AiConfig {
    /// Path to the baseline AI configuration files
    #[serde(default = "default_ai_config_dir")]
    pub config_dir: PathBuf,

    /// Tools to configure
    #[serde(default = "default_ai_tools")]
    pub tools: Vec<String>,
}

/// Version tracking information
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VersionsConfig {
    /// Last checked timestamp (ISO 8601 format)
    #[serde(default)]
    pub last_checked: Option<String>,

    /// Package managers
    #[serde(default)]
    pub package_managers: HashMap<String, PackageManagerVersions>,

    /// GitHub Actions
    #[serde(default)]
    pub github_actions: HashMap<String, String>,

    /// Tools
    #[serde(default)]
    pub tools: HashMap<String, String>,
}

/// Package manager versions
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PackageManagerVersions {
    /// Latest versions of packages
    #[serde(default)]
    pub packages: HashMap<String, String>,
}

// Default function implementations
fn default_baseline_path() -> PathBuf {
    PathBuf::from("/Users/dave/src/github.com/daveio/_baseline")
}

fn default_templates_dir() -> PathBuf {
    PathBuf::from("templates")
}

fn default_ai_config_dir() -> PathBuf {
    default_baseline_path()
}

fn default_ai_tools() -> Vec<String> {
    vec![
        "claude-desktop".to_string(),
        "cursor".to_string(),
        "zed".to_string(),
        "goose".to_string(),
    ]
}

/// Configuration manager
pub struct ConfigManager {
    app_config: AppConfig,
    versions_config: VersionsConfig,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Result<Self> {
        let app_config = Self::load_app_config()?;
        let versions_config = Self::load_versions_config()?;

        Ok(Self {
            app_config,
            versions_config,
        })
    }

    /// Get the application configuration
    pub fn app_config(&self) -> &AppConfig {
        &self.app_config
    }

    /// Get the versions configuration
    pub fn versions_config(&self) -> &VersionsConfig {
        &self.versions_config
    }

    /// Get mutable reference to versions configuration
    pub fn versions_config_mut(&mut self) -> &mut VersionsConfig {
        &mut self.versions_config
    }

    /// Save all configurations
    pub fn save(&self) -> Result<()> {
        self.save_app_config()?;
        self.save_versions_config()?;
        Ok(())
    }

    /// Save only the versions configuration
    pub fn save_versions(&self) -> Result<()> {
        self.save_versions_config()
    }

    /// Load application configuration
    fn load_app_config() -> Result<AppConfig> {
        let config_path = Self::get_app_config_path()?;
        let config_dir = config_path.parent().unwrap();

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).context(format!(
                "Failed to create config directory at {}",
                config_dir.display()
            ))?;
        }

        // Build the configuration
        let config = Config::builder()
            .add_source(File::from(config_path).required(false))
            .add_source(Environment::with_prefix("JIG"))
            .build()
            .context("Failed to build configuration")?;

        let app_config: AppConfig = config
            .try_deserialize()
            .context("Failed to deserialize configuration")?;

        debug!("Loaded application configuration");

        Ok(app_config)
    }

    /// Load versions configuration
    fn load_versions_config() -> Result<VersionsConfig> {
        let versions_path = Self::get_versions_config_path()?;
        let config_dir = versions_path.parent().unwrap();

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).context(format!(
                "Failed to create config directory at {}",
                config_dir.display()
            ))?;
        }

        if !versions_path.exists() {
            debug!("Versions file not found, using default configuration");
            return Ok(VersionsConfig::default());
        }

        // Build the configuration
        let config = Config::builder()
            .add_source(File::from(versions_path).required(false))
            .build()
            .context("Failed to build versions configuration")?;

        let versions_config: VersionsConfig = config
            .try_deserialize()
            .context("Failed to deserialize versions configuration")?;

        debug!("Loaded versions configuration");

        Ok(versions_config)
    }

    /// Save application configuration
    fn save_app_config(&self) -> Result<()> {
        let config_path = Self::get_app_config_path()?;
        Self::ensure_parent_dir(&config_path)?;

        // Convert to YAML using yaml-rust2
        let yaml = self
            .serialize_to_yaml(&self.app_config)
            .context("Failed to serialize application configuration")?;

        fs::write(&config_path, yaml).context(format!(
            "Failed to write application config to {}",
            config_path.display()
        ))?;

        debug!(
            "Saved application configuration to {}",
            config_path.display()
        );
        Ok(())
    }

    /// Save versions configuration
    fn save_versions_config(&self) -> Result<()> {
        let versions_path = Self::get_versions_config_path()?;
        Self::ensure_parent_dir(&versions_path)?;

        // Convert to YAML using yaml-rust2
        let yaml = self
            .serialize_to_yaml(&self.versions_config)
            .context("Failed to serialize versions configuration")?;

        fs::write(&versions_path, yaml).context(format!(
            "Failed to write versions config to {}",
            versions_path.display()
        ))?;

        debug!(
            "Saved versions configuration to {}",
            versions_path.display()
        );
        Ok(())
    }

    /// Helper method to serialize a struct to YAML using yaml-rust2
    fn serialize_to_yaml<T: Serialize>(&self, value: &T) -> Result<String> {
        // First serialize to JSON using serde_json
        let json = serde_json::to_value(value).context("Failed to serialize to JSON")?;

        // Convert JSON to yaml-rust2 Yaml format
        let yaml_docs =
            YamlLoader::load_from_str(&json.to_string()).context("Failed to parse JSON as YAML")?;

        if yaml_docs.is_empty() {
            return Err(anyhow!("Failed to convert JSON to YAML"));
        }

        // Output the YAML as a string
        let mut yaml_string = String::new();
        let mut emitter = YamlEmitter::new(&mut yaml_string);
        emitter.dump(&yaml_docs[0]).context("Failed to emit YAML")?;

        Ok(yaml_string)
    }

    /// Initialize configuration if not exists (typically called when --help is used)
    pub fn initialize_if_not_exists() -> Result<()> {
        let app_config_path = Self::get_app_config_path()?;
        let versions_config_path = Self::get_versions_config_path()?;
        let config_dir = app_config_path.parent().unwrap();

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).context(format!(
                "Failed to create config directory at {}",
                config_dir.display()
            ))?;

            info!(
                "Created configuration directory at {}",
                config_dir.display()
            );
        }

        // Create default app config if it doesn't exist
        if !app_config_path.exists() {
            let default_config = AppConfig {
                baseline_path: default_baseline_path(),
                templates: TemplatesConfig::default(),
                ai: AiConfig::default(),
            };

            // Convert to YAML using yaml-rust2
            let config_manager = ConfigManager {
                app_config: default_config.clone(),
                versions_config: VersionsConfig::default(),
            };

            let yaml = config_manager
                .serialize_to_yaml(&default_config)
                .context("Failed to serialize default application configuration")?;

            fs::write(&app_config_path, yaml).context(format!(
                "Failed to write default application config to {}",
                app_config_path.display()
            ))?;

            info!(
                "Created default application configuration at {}",
                app_config_path.display()
            );
        }

        // Create default versions config if it doesn't exist
        if !versions_config_path.exists() {
            let default_versions = VersionsConfig::default();

            // Convert to YAML using yaml-rust2
            let config_manager = ConfigManager {
                app_config: AppConfig {
                    baseline_path: default_baseline_path(),
                    templates: TemplatesConfig::default(),
                    ai: AiConfig::default(),
                },
                versions_config: default_versions.clone(),
            };

            let yaml = config_manager
                .serialize_to_yaml(&default_versions)
                .context("Failed to serialize default versions configuration")?;

            fs::write(&versions_config_path, yaml).context(format!(
                "Failed to write default versions config to {}",
                versions_config_path.display()
            ))?;

            info!(
                "Created default versions configuration at {}",
                versions_config_path.display()
            );
        }

        Ok(())
    }

    /// Get the path to the application configuration file
    fn get_app_config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().context("Failed to get home directory")?;
        Ok(home_dir.join(".config/jig/config.yaml"))
    }

    /// Get the path to the versions configuration file
    fn get_versions_config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().context("Failed to get home directory")?;
        Ok(home_dir.join(".config/jig/versions.yaml"))
    }

    /// Ensure the parent directory exists
    fn ensure_parent_dir(path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).context(format!(
                    "Failed to create directory at {}",
                    parent.display()
                ))?;
            }
        }
        Ok(())
    }
}

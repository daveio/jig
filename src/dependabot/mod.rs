use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct DependabotConfig {
    pub version: u32,
    pub updates: Vec<UpdateConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConfig {
    #[serde(rename = "package-ecosystem")]
    pub package_ecosystem: String,
    pub directory: String,
    pub schedule: ScheduleConfig,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub interval: String,
}

/// Detect package ecosystems in a repository
pub fn detect_ecosystems(directory: &Path) -> Result<HashSet<String>> {
    let mut ecosystems = HashSet::new();

    // Map of file patterns to ecosystems
    let ecosystem_patterns: HashMap<&str, Vec<&str>> = HashMap::from([
        ("package.json", vec!["npm"]),
        ("package-lock.json", vec!["npm"]),
        ("yarn.lock", vec!["npm"]),
        ("pnpm-lock.yaml", vec!["npm"]),
        ("bun.lockb", vec!["npm"]),
        ("Gemfile", vec!["bundler"]),
        ("Gemfile.lock", vec!["bundler"]),
        ("pyproject.toml", vec!["pip"]),
        ("requirements.txt", vec!["pip"]),
        ("Cargo.toml", vec!["cargo"]),
        ("go.mod", vec!["gomod"]),
        ("Dockerfile", vec!["docker"]),
        ("composer.json", vec!["composer"]),
        ("pubspec.yaml", vec!["pub"]),
        ("build.gradle", vec!["gradle"]),
        ("pom.xml", vec!["maven"]),
        ("build.sbt", vec!["sbt"]),
        ("mix.exs", vec!["mix"]),
        ("elm.json", vec!["elm"]),
        ("nuget.config", vec!["nuget"]),
    ]);

    // Walk through the directory
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_entry(|e| !e.path().components().any(|c| c.as_os_str() == ".git"))
    {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(filename) = entry.file_name().to_str() {
                if let Some(eco_list) = ecosystem_patterns.get(filename) {
                    for ecosystem in eco_list {
                        ecosystems.insert(ecosystem.to_string());
                    }
                }
            }
        }
    }

    // Check for GitHub Actions workflows
    let github_workflows_dir = directory.join(".github").join("workflows");
    if github_workflows_dir.exists() && github_workflows_dir.is_dir() {
        let has_workflows = fs::read_dir(&github_workflows_dir)?.any(|entry| {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    return name.ends_with(".yml") || name.ends_with(".yaml");
                }
            }
            false
        });

        if has_workflows {
            ecosystems.insert("github-actions".to_string());
        }
    }

    Ok(ecosystems)
}

/// Find dependabot configuration file
pub fn find_dependabot_config(directory: &Path) -> Option<PathBuf> {
    let config_path = directory.join(".github").join("dependabot.yml");
    if config_path.exists() {
        return Some(config_path);
    }

    let alt_config_path = directory.join(".github").join("dependabot.yaml");
    if alt_config_path.exists() {
        return Some(alt_config_path);
    }

    None
}

/// Update or create dependabot configuration
pub fn update_dependabot_config(
    directory: &Path,
    dry_run: bool,
    assignees: Vec<String>,
) -> Result<Vec<String>> {
    let mut added_ecosystems = Vec::new();

    // Detect ecosystems in the repository
    let detected_ecosystems = detect_ecosystems(directory)?;

    // Find or create config path
    let config_path = find_dependabot_config(directory)
        .unwrap_or_else(|| directory.join(".github").join("dependabot.yml"));

    // Load existing config or create default
    let mut config = if config_path.exists() {
        let content =
            fs::read_to_string(&config_path).context("Failed to read dependabot config")?;
        serde_yaml::from_str::<DependabotConfig>(&content)
            .context("Failed to parse dependabot config")?
    } else {
        DependabotConfig {
            version: 2,
            updates: Vec::new(),
        }
    };

    // Get existing ecosystems
    let existing_ecosystems: HashSet<String> = config
        .updates
        .iter()
        .map(|update| update.package_ecosystem.clone())
        .collect();

    // Add missing ecosystems
    for ecosystem in detected_ecosystems {
        if !existing_ecosystems.contains(&ecosystem) {
            added_ecosystems.push(ecosystem.clone());

            if !dry_run {
                config.updates.push(UpdateConfig {
                    package_ecosystem: ecosystem,
                    directory: "/".to_string(),
                    schedule: ScheduleConfig {
                        interval: "daily".to_string(),
                    },
                    assignees: assignees.clone(),
                });
            }
        }
    }

    // Write updated config
    if !added_ecosystems.is_empty() && !dry_run {
        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content =
            serde_yaml::to_string(&config).context("Failed to serialize dependabot config")?;
        fs::write(&config_path, content).context("Failed to write dependabot config")?;
    }

    Ok(added_ecosystems)
}

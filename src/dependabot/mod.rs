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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_detect_ecosystems_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let ecosystems = detect_ecosystems(temp_dir.path()).unwrap();
        assert!(ecosystems.is_empty());
    }

    #[test]
    fn test_detect_ecosystems_rust() {
        let temp_dir = tempdir().unwrap();
        let cargo_toml = temp_dir.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"").unwrap();

        let ecosystems = detect_ecosystems(temp_dir.path()).unwrap();
        assert!(ecosystems.contains("cargo"));
        assert_eq!(ecosystems.len(), 1);
    }

    #[test]
    fn test_detect_ecosystems_javascript() {
        let temp_dir = tempdir().unwrap();
        let package_json = temp_dir.path().join("package.json");
        fs::write(&package_json, "{\"name\": \"test\"}").unwrap();

        let ecosystems = detect_ecosystems(temp_dir.path()).unwrap();
        assert!(ecosystems.contains("npm"));
        assert_eq!(ecosystems.len(), 1);
    }

    #[test]
    fn test_detect_ecosystems_python() {
        let temp_dir = tempdir().unwrap();
        let requirements_txt = temp_dir.path().join("requirements.txt");
        fs::write(&requirements_txt, "requests==2.25.1").unwrap();

        let ecosystems = detect_ecosystems(temp_dir.path()).unwrap();
        assert!(ecosystems.contains("pip"));
        assert_eq!(ecosystems.len(), 1);
    }

    #[test]
    fn test_detect_ecosystems_github_actions() {
        let temp_dir = tempdir().unwrap();
        let workflows_dir = temp_dir.path().join(".github").join("workflows");
        fs::create_dir_all(&workflows_dir).unwrap();

        let workflow_file = workflows_dir.join("ci.yml");
        fs::write(&workflow_file, "name: CI\non: [push]").unwrap();

        let ecosystems = detect_ecosystems(temp_dir.path()).unwrap();
        assert!(ecosystems.contains("github-actions"));
        assert_eq!(ecosystems.len(), 1);
    }

    #[test]
    fn test_detect_ecosystems_multiple() {
        let temp_dir = tempdir().unwrap();

        // Create multiple ecosystem files
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();
        fs::write(temp_dir.path().join("package.json"), "{\"name\": \"test\"}").unwrap();
        fs::write(temp_dir.path().join("requirements.txt"), "requests==2.25.1").unwrap();

        let ecosystems = detect_ecosystems(temp_dir.path()).unwrap();
        assert!(ecosystems.contains("cargo"));
        assert!(ecosystems.contains("npm"));
        assert!(ecosystems.contains("pip"));
        assert_eq!(ecosystems.len(), 3);
    }

    #[test]
    fn test_detect_ecosystems_ignores_git_directory() {
        let temp_dir = tempdir().unwrap();

        // Create .git directory with files
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir_all(&git_dir).unwrap();
        fs::write(git_dir.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();

        // Create actual project file
        fs::write(temp_dir.path().join("package.json"), "{\"name\": \"test\"}").unwrap();

        let ecosystems = detect_ecosystems(temp_dir.path()).unwrap();
        assert!(ecosystems.contains("npm"));
        assert!(!ecosystems.contains("cargo")); // Should be ignored from .git directory
        assert_eq!(ecosystems.len(), 1);
    }

    #[test]
    fn test_find_dependabot_config_yml() {
        let temp_dir = tempdir().unwrap();
        let github_dir = temp_dir.path().join(".github");
        fs::create_dir_all(&github_dir).unwrap();

        let config_path = github_dir.join("dependabot.yml");
        fs::write(&config_path, "version: 2").unwrap();

        let found_config = find_dependabot_config(temp_dir.path());
        assert!(found_config.is_some());
        assert_eq!(found_config.unwrap(), config_path);
    }

    #[test]
    fn test_find_dependabot_config_yaml() {
        let temp_dir = tempdir().unwrap();
        let github_dir = temp_dir.path().join(".github");
        fs::create_dir_all(&github_dir).unwrap();

        let config_path = github_dir.join("dependabot.yaml");
        fs::write(&config_path, "version: 2").unwrap();

        let found_config = find_dependabot_config(temp_dir.path());
        assert!(found_config.is_some());
        assert_eq!(found_config.unwrap(), config_path);
    }

    #[test]
    fn test_find_dependabot_config_prefers_yml() {
        let temp_dir = tempdir().unwrap();
        let github_dir = temp_dir.path().join(".github");
        fs::create_dir_all(&github_dir).unwrap();

        let yml_path = github_dir.join("dependabot.yml");
        let yaml_path = github_dir.join("dependabot.yaml");
        fs::write(&yml_path, "version: 2").unwrap();
        fs::write(&yaml_path, "version: 2").unwrap();

        let found_config = find_dependabot_config(temp_dir.path());
        assert!(found_config.is_some());
        assert_eq!(found_config.unwrap(), yml_path); // Should prefer .yml
    }

    #[test]
    fn test_find_dependabot_config_none() {
        let temp_dir = tempdir().unwrap();
        let found_config = find_dependabot_config(temp_dir.path());
        assert!(found_config.is_none());
    }

    #[test]
    fn test_update_dependabot_config_new_file() {
        let temp_dir = tempdir().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let assignees = vec!["user1".to_string()];
        let added = update_dependabot_config(temp_dir.path(), false, assignees.clone()).unwrap();

        assert!(added.contains(&"cargo".to_string()));
        assert_eq!(added.len(), 1);

        // Verify file was created
        let config_path = temp_dir.path().join(".github").join("dependabot.yml");
        assert!(config_path.exists());

        // Verify content
        let content = fs::read_to_string(&config_path).unwrap();
        let config: DependabotConfig = serde_yaml::from_str(&content).unwrap();
        assert_eq!(config.version, 2);
        assert_eq!(config.updates.len(), 1);
        assert_eq!(config.updates[0].package_ecosystem, "cargo");
        assert_eq!(config.updates[0].assignees, assignees);
    }

    #[test]
    fn test_update_dependabot_config_existing_file() {
        let temp_dir = tempdir().unwrap();
        let github_dir = temp_dir.path().join(".github");
        fs::create_dir_all(&github_dir).unwrap();

        // Create existing config
        let config_path = github_dir.join("dependabot.yml");
        let existing_config = r#"version: 2
updates:
  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "daily"
    assignees: []
"#;
        fs::write(&config_path, existing_config).unwrap();

        // Add new ecosystem
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let added = update_dependabot_config(temp_dir.path(), false, vec![]).unwrap();

        assert!(added.contains(&"cargo".to_string()));
        assert!(!added.contains(&"npm".to_string())); // Should not add existing
        assert_eq!(added.len(), 1);

        // Verify content
        let content = fs::read_to_string(&config_path).unwrap();
        let config: DependabotConfig = serde_yaml::from_str(&content).unwrap();
        assert_eq!(config.version, 2);
        assert_eq!(config.updates.len(), 2); // npm + cargo
    }

    #[test]
    fn test_update_dependabot_config_dry_run() {
        let temp_dir = tempdir().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let added = update_dependabot_config(temp_dir.path(), true, vec![]).unwrap();

        assert!(added.contains(&"cargo".to_string()));
        assert_eq!(added.len(), 1);

        // Verify file was NOT created
        let config_path = temp_dir.path().join(".github").join("dependabot.yml");
        assert!(!config_path.exists());
    }

    #[test]
    fn test_dependabot_config_serialization() {
        let config = DependabotConfig {
            version: 2,
            updates: vec![
                UpdateConfig {
                    package_ecosystem: "npm".to_string(),
                    directory: "/".to_string(),
                    schedule: ScheduleConfig {
                        interval: "daily".to_string(),
                    },
                    assignees: vec!["user1".to_string()],
                },
                UpdateConfig {
                    package_ecosystem: "cargo".to_string(),
                    directory: "/".to_string(),
                    schedule: ScheduleConfig {
                        interval: "weekly".to_string(),
                    },
                    assignees: vec!["user2".to_string()],
                },
            ],
        };

        let yaml_str = serde_yaml::to_string(&config).unwrap();
        assert!(yaml_str.contains("version: 2"));
        assert!(yaml_str.contains("package-ecosystem: npm"));
        assert!(yaml_str.contains("package-ecosystem: cargo"));
        assert!(yaml_str.contains("interval: daily"));
        assert!(yaml_str.contains("interval: weekly"));

        // Test deserialization
        let deserialized: DependabotConfig = serde_yaml::from_str(&yaml_str).unwrap();
        assert_eq!(deserialized.version, config.version);
        assert_eq!(deserialized.updates.len(), config.updates.len());
        assert_eq!(deserialized.updates[0].package_ecosystem, "npm");
        assert_eq!(deserialized.updates[1].package_ecosystem, "cargo");
    }
}

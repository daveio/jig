use super::PackageManagerResult;
use crate::config::{PackageManagerVersions, VersionsConfig};
use anyhow::{anyhow, Context, Result};
use log::{debug, info, warn};
use regex::Regex;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::{Table, Value};

#[derive(Debug, Deserialize)]
struct PypiPackageInfo {
    info: PypiPackageInfoData,
}

#[derive(Debug, Deserialize)]
struct PypiPackageInfoData {
    version: String,
}

/// Bump Python package versions
pub fn bump_versions(repo_path: &Path) -> Result<PackageManagerResult> {
    debug!(
        "Checking for Python package managers in: {}",
        repo_path.display()
    );

    let mut result = PackageManagerResult {
        name: "Python".to_string(),
        updated_files: Vec::new(),
        errors: Vec::new(),
    };

    // Check for pyproject.toml
    let pyproject_path = repo_path.join("pyproject.toml");
    if pyproject_path.exists() {
        debug!("Found pyproject.toml");

        match update_pyproject_toml(&pyproject_path) {
            Ok(updated) => {
                if updated {
                    result.updated_files.push(pyproject_path);
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to update pyproject.toml: {}", e));
            }
        }
    }

    // Check for requirements.txt
    let requirements_path = repo_path.join("requirements.txt");
    if requirements_path.exists() {
        debug!("Found requirements.txt");

        match update_requirements_txt(&requirements_path) {
            Ok(updated) => {
                if updated {
                    result.updated_files.push(requirements_path);

                    // If we're using a requirements.txt, ensure we have uv
                    migrate_to_uv(repo_path, &mut result)?;
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to update requirements.txt: {}", e));
            }
        }
    }

    // Check for setup.py
    let setup_py_path = repo_path.join("setup.py");
    if setup_py_path.exists() {
        debug!("Found setup.py");

        // Recommend migrating to pyproject.toml
        info!("setup.py found. Consider migrating to pyproject.toml with PEP-518 keys");

        // Attempt migration
        match migrate_to_pyproject_toml(repo_path, &setup_py_path) {
            Ok(migrated) => {
                if migrated {
                    result.updated_files.push(repo_path.join("pyproject.toml"));
                }
            }
            Err(e) => {
                result.errors.push(format!(
                    "Failed to migrate setup.py to pyproject.toml: {}",
                    e
                ));
            }
        }
    }

    // Check for poetry
    let poetry_lock_path = repo_path.join("poetry.lock");
    if poetry_lock_path.exists() {
        debug!("Found poetry.lock");

        // Recommend migrating to uv
        info!("poetry.lock found. Attempting to migrate to uv");

        migrate_to_uv(repo_path, &mut result)?;
    }

    Ok(result)
}

/// Bump Python package versions using cached version information
pub fn bump_versions_from_cache(
    repo_path: &Path,
    versions_config: &VersionsConfig,
) -> Result<PackageManagerResult> {
    debug!(
        "Checking for Python package managers using cached versions in: {}",
        repo_path.display()
    );

    let mut result = PackageManagerResult {
        name: "Python".to_string(),
        updated_files: Vec::new(),
        errors: Vec::new(),
    };

    // Check if we have cached Python versions
    if !versions_config.package_managers.contains_key("python") {
        debug!("No cached Python versions found");
        return Ok(result);
    }

    let python_versions = match versions_config.package_managers.get("python") {
        Some(versions) => versions,
        None => {
            debug!("No cached Python versions found");
            return Ok(result);
        }
    };

    // Check for pyproject.toml
    let pyproject_path = repo_path.join("pyproject.toml");
    if pyproject_path.exists() {
        debug!("Found pyproject.toml");

        match update_pyproject_toml_from_cache(&pyproject_path, python_versions) {
            Ok(updated) => {
                if updated {
                    result.updated_files.push(pyproject_path);
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to update pyproject.toml from cache: {}", e));
            }
        }
    }

    // Check for requirements.txt
    let requirements_path = repo_path.join("requirements.txt");
    if requirements_path.exists() {
        debug!("Found requirements.txt");

        match update_requirements_txt_from_cache(&requirements_path, python_versions) {
            Ok(updated) => {
                if updated {
                    result.updated_files.push(requirements_path);
                }
            }
            Err(e) => {
                result.errors.push(format!(
                    "Failed to update requirements.txt from cache: {}",
                    e
                ));
            }
        }
    }

    Ok(result)
}

/// Update pyproject.toml with latest versions
fn update_pyproject_toml(path: &Path) -> Result<bool> {
    debug!("Updating pyproject.toml at: {}", path.display());

    let content = fs::read_to_string(path)?;
    let mut doc = content
        .parse::<Table>()
        .context("Failed to parse pyproject.toml")?;

    let mut updated = false;
    let mut package_versions: HashMap<String, String> = HashMap::new();

    // Update dependencies in [project]
    if let Some(Value::Table(project)) = doc.get_mut("project") {
        if let Some(Value::Table(dependencies)) = project.get_mut("dependencies") {
            updated |= update_pyproject_dependencies(dependencies, &mut package_versions)?;
        }
    }

    // Also update any tool.poetry dependencies
    if let Some(Value::Table(tool)) = doc.get_mut("tool") {
        if let Some(Value::Table(poetry)) = tool.get_mut("poetry") {
            if let Some(Value::Table(dependencies)) = poetry.get_mut("dependencies") {
                updated |= update_pyproject_dependencies(dependencies, &mut package_versions)?;
            }
        }
    }

    if updated {
        // Write the updated TOML back to the file
        fs::write(path, doc.to_string())?;
        info!("Updated pyproject.toml with latest versions");
    }

    Ok(updated)
}

/// Update pyproject.toml with cached versions
fn update_pyproject_toml_from_cache(
    path: &Path,
    python_versions: &PackageManagerVersions,
) -> Result<bool> {
    debug!("Updating pyproject.toml from cache at: {}", path.display());

    let content = fs::read_to_string(path)?;
    let mut doc = content
        .parse::<Table>()
        .context("Failed to parse pyproject.toml")?;

    let mut updated = false;

    // Update dependencies in [project]
    if let Some(Value::Table(project)) = doc.get_mut("project") {
        if let Some(Value::Table(dependencies)) = project.get_mut("dependencies") {
            updated |= update_pyproject_dependencies_from_cache(dependencies, python_versions)?;
        }
    }

    // Also update any tool.poetry dependencies
    if let Some(Value::Table(tool)) = doc.get_mut("tool") {
        if let Some(Value::Table(poetry)) = tool.get_mut("poetry") {
            if let Some(Value::Table(dependencies)) = poetry.get_mut("dependencies") {
                updated |= update_pyproject_dependencies_from_cache(dependencies, python_versions)?;
            }
        }
    }

    if updated {
        // Write the updated TOML back to the file
        fs::write(path, doc.to_string())?;
        info!("Updated pyproject.toml with cached versions");
    }

    Ok(updated)
}

/// Update dependencies in a pyproject.toml section
fn update_pyproject_dependencies(
    dependencies: &mut Table,
    package_versions: &mut HashMap<String, String>,
) -> Result<bool> {
    let mut updated = false;
    let mut deps_to_update = Vec::new();

    // First collect all dependencies we need to update
    for (name, value) in dependencies.iter() {
        if let Value::String(version) = value {
            if version.contains("==") || version.contains(">=") || version.contains("~=") {
                deps_to_update.push(name.clone());
            }
        }
    }

    // Then update them with the latest versions
    for name in deps_to_update {
        let latest_version = match package_versions.get(&name) {
            Some(version) => version.clone(),
            None => {
                // Fetch latest version from PyPI
                match get_latest_version(&name) {
                    Ok(version) => {
                        package_versions.insert(name.clone(), version.clone());
                        version
                    }
                    Err(e) => {
                        warn!("Failed to get latest version for {}: {}", name, e);
                        continue;
                    }
                }
            }
        };

        // Update the dependency
        if let Some(Value::String(version_mut)) = dependencies.get_mut(&name) {
            *version_mut = latest_version;
            updated = true;
        }
    }

    Ok(updated)
}

/// Update dependencies in a pyproject.toml section from cache
fn update_pyproject_dependencies_from_cache(
    dependencies: &mut Table,
    python_versions: &PackageManagerVersions,
) -> Result<bool> {
    let mut updated = false;

    for (name, value) in dependencies.iter_mut() {
        // Check if we have a cached version for this package
        if let Some(cached_version) = python_versions.packages.get(name) {
            if let Value::String(version) = value {
                if version.contains("==") || version.contains(">=") || version.contains("~=") {
                    *version = cached_version.to_string();
                    updated = true;
                }
            }
        }
    }

    Ok(updated)
}

/// Update requirements.txt with latest versions
fn update_requirements_txt(path: &Path) -> Result<bool> {
    debug!("Updating requirements.txt at: {}", path.display());

    let content = fs::read_to_string(path)?;
    let lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();

    let mut updated_lines = Vec::new();
    let mut updated = false;
    let mut package_versions: HashMap<String, String> = HashMap::new();

    // Regular expression to match package specifications with versions
    let re = Regex::new(r"^([a-zA-Z0-9_.-]+)([=~<>!]+.+)$")?;

    for line in lines {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            // Keep comments and empty lines unchanged
            updated_lines.push(line);
            continue;
        }

        if re.is_match(&line) {
            let captures = re.captures(&line).unwrap();
            let package_name = captures.get(1).unwrap().as_str();

            // Get latest version for this package
            let latest_version = match package_versions.get(package_name) {
                Some(version) => version.clone(),
                None => {
                    // Fetch latest version from PyPI
                    match get_latest_version(package_name) {
                        Ok(version) => {
                            package_versions.insert(package_name.to_string(), version.clone());
                            version
                        }
                        Err(e) => {
                            warn!("Failed to get latest version for {}: {}", package_name, e);
                            updated_lines.push(line);
                            continue;
                        }
                    }
                }
            };

            // Update the line with the latest version
            let updated_line = format!("{}=={}", package_name, latest_version);
            updated_lines.push(updated_line);
            updated = true;
        } else {
            updated_lines.push(line);
        }
    }

    if updated {
        // Write the updated content back to the file
        fs::write(path, updated_lines.join("\n"))?;
        info!("Updated requirements.txt with latest versions");
    }

    Ok(updated)
}

/// Update requirements.txt with cached versions
fn update_requirements_txt_from_cache(
    path: &Path,
    python_versions: &PackageManagerVersions,
) -> Result<bool> {
    debug!(
        "Updating requirements.txt from cache at: {}",
        path.display()
    );

    let content = fs::read_to_string(path)?;
    let lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();

    let mut updated_lines = Vec::new();
    let mut updated = false;

    // Regular expression to match package specifications with versions
    let re = Regex::new(r"^([a-zA-Z0-9_.-]+)([=~<>!]+.+)$")?;

    for line in lines {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            // Keep comments and empty lines unchanged
            updated_lines.push(line);
            continue;
        }

        if re.is_match(&line) {
            let captures = re.captures(&line).unwrap();
            let package_name = captures.get(1).unwrap().as_str();

            // Check if we have a cached version for this package
            if let Some(cached_version) = python_versions.packages.get(package_name) {
                // Update the line with the cached version
                let updated_line = format!("{}=={}", package_name, cached_version);
                updated_lines.push(updated_line);
                updated = true;
            } else {
                updated_lines.push(line);
            }
        } else {
            updated_lines.push(line);
        }
    }

    if updated {
        // Write the updated content back to the file
        fs::write(path, updated_lines.join("\n"))?;
        info!("Updated requirements.txt with cached versions");
    }

    Ok(updated)
}

/// Get the latest version of a package from PyPI
fn get_latest_version(package_name: &str) -> Result<String> {
    debug!(
        "Getting latest version for Python package: {}",
        package_name
    );

    let client = Client::new();
    let url = format!("https://pypi.org/pypi/{}/json", package_name);

    let response = client.get(&url).header("User-Agent", "jig-tool").send()?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to get package info: HTTP {}",
            response.status()
        ));
    }

    let package_data: PypiPackageInfo = response.json()?;
    let version = package_data.info.version;

    debug!("Latest version of {} is {}", package_name, version);

    Ok(version)
}

/// Migrate from other Python package managers to uv
fn migrate_to_uv(repo_path: &Path, result: &mut PackageManagerResult) -> Result<()> {
    debug!("Migrating to uv");

    // Check if pyproject.toml exists
    let pyproject_path = repo_path.join("pyproject.toml");
    let mut pyproject_content = if pyproject_path.exists() {
        fs::read_to_string(&pyproject_path)?
    } else {
        String::new()
    };

    // Add or update [build-system] section for uv
    if !pyproject_content.contains("[build-system]") {
        // If no build-system section, add it
        pyproject_content.push_str("\n[build-system]\n");
        pyproject_content.push_str("requires = [\"hatchling\"]\n");
        pyproject_content.push_str("build-backend = \"hatchling.build\"\n");
    } else {
        // Otherwise, keep existing build system as uv is compatible with PEP-517
        debug!("Keeping existing build-system section");
    }

    // Write updated pyproject.toml
    fs::write(&pyproject_path, pyproject_content)?;
    info!("Updated pyproject.toml for uv compatibility");

    // Add the file to updated files if it's not already there
    if !result.updated_files.contains(&pyproject_path) {
        result.updated_files.push(pyproject_path);
    }

    // Create .python-version if it doesn't exist
    let python_version_path = repo_path.join(".python-version");
    if !python_version_path.exists() {
        fs::write(&python_version_path, "3.11.0\n")?;
        info!("Created .python-version file");
        result.updated_files.push(python_version_path);
    }

    Ok(())
}

/// Migrate from setup.py to pyproject.toml
fn migrate_to_pyproject_toml(repo_path: &Path, _setup_py_path: &Path) -> Result<bool> {
    debug!("Attempting to migrate from setup.py to pyproject.toml");

    // Simple migration - create a basic pyproject.toml if it doesn't exist
    let pyproject_path = repo_path.join("pyproject.toml");
    if pyproject_path.exists() {
        debug!("pyproject.toml already exists, skipping migration");
        return Ok(false);
    }

    // Basic pyproject.toml template
    let content = r#"[build-system]
requires = ["setuptools>=42", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "project-name"
version = "0.1.0"
description = "Project description"
requires-python = ">=3.7"
dependencies = []

[tool.setuptools]
packages = ["src"]
"#;

    fs::write(&pyproject_path, content)?;
    info!("Created basic pyproject.toml. Manual adjustments may be needed.");

    Ok(true)
}

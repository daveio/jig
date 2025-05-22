use super::PackageManagerResult;
use crate::config::{PackageManagerVersions, VersionsConfig};
use anyhow::{anyhow, Context, Result};
use log::{debug, info, warn};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml::{Table, Value};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CratesIoResponse {
    #[serde(rename = "crate")]
    crate_info: CrateInfo,
    versions: Vec<CrateVersion>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CrateInfo {
    name: String,
}

#[derive(Debug, Deserialize)]
struct CrateVersion {
    num: String,
    yanked: bool,
}

/// Bump Rust package versions
pub fn bump_versions(repo_path: &Path) -> Result<PackageManagerResult> {
    debug!(
        "Checking for Rust package managers in: {}",
        repo_path.display()
    );

    let mut result = PackageManagerResult {
        name: "Rust".to_string(),
        updated_files: Vec::new(),
        errors: Vec::new(),
    };

    // Check for Cargo.toml
    let cargo_toml_path = repo_path.join("Cargo.toml");
    if cargo_toml_path.exists() {
        debug!("Found Cargo.toml");

        match update_cargo_toml(&cargo_toml_path) {
            Ok(updated) => {
                if updated {
                    result.updated_files.push(cargo_toml_path.clone());
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to update Cargo.toml: {}", e));
            }
        }

        // Check for Cargo.toml files in workspace members
        match find_workspace_members(repo_path, &cargo_toml_path) {
            Ok(members) => {
                for member_path in members {
                    match update_cargo_toml(&member_path) {
                        Ok(updated) => {
                            if updated {
                                result.updated_files.push(member_path);
                            }
                        }
                        Err(e) => {
                            result
                                .errors
                                .push(format!("Failed to update member Cargo.toml: {}", e));
                        }
                    }
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to find workspace members: {}", e));
            }
        }
    }

    Ok(result)
}

/// Bump Rust package versions using cached version information
pub fn bump_versions_from_cache(
    repo_path: &Path,
    versions_config: &VersionsConfig,
) -> Result<PackageManagerResult> {
    debug!(
        "Checking for Rust package managers using cached versions in: {}",
        repo_path.display()
    );

    let mut result = PackageManagerResult {
        name: "Rust".to_string(),
        updated_files: Vec::new(),
        errors: Vec::new(),
    };

    // Check if we have cached Rust versions
    if !versions_config.package_managers.contains_key("rust") {
        debug!("No cached Rust versions found");
        return Ok(result);
    }

    let rust_versions = match versions_config.package_managers.get("rust") {
        Some(versions) => versions,
        None => {
            debug!("No cached Rust versions found");
            return Ok(result);
        }
    };

    // Check for Cargo.toml
    let cargo_toml_path = repo_path.join("Cargo.toml");
    if cargo_toml_path.exists() {
        debug!("Found Cargo.toml");

        match update_cargo_toml_from_cache(&cargo_toml_path, rust_versions) {
            Ok(updated) => {
                if updated {
                    result.updated_files.push(cargo_toml_path.clone());
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to update Cargo.toml from cache: {}", e));
            }
        }

        // Check for Cargo.toml files in workspace members
        match find_workspace_members(repo_path, &cargo_toml_path) {
            Ok(members) => {
                for member_path in members {
                    match update_cargo_toml_from_cache(&member_path, rust_versions) {
                        Ok(updated) => {
                            if updated {
                                result.updated_files.push(member_path);
                            }
                        }
                        Err(e) => {
                            result.errors.push(format!(
                                "Failed to update member Cargo.toml from cache: {}",
                                e
                            ));
                        }
                    }
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to find workspace members: {}", e));
            }
        }
    }

    Ok(result)
}

/// Update Cargo.toml with latest versions
fn update_cargo_toml(path: &Path) -> Result<bool> {
    debug!("Updating Cargo.toml at: {}", path.display());

    let content = fs::read_to_string(path)?;
    let mut doc = content
        .parse::<Table>()
        .context("Failed to parse Cargo.toml")?;

    let mut updated = false;
    let mut package_versions = HashMap::new();

    // Update dependencies
    if let Some(Value::Table(dependencies)) = doc.get_mut("dependencies") {
        updated |= update_dependencies_table(dependencies, &mut package_versions)?;
    }

    // Update dev-dependencies
    if let Some(Value::Table(dev_dependencies)) = doc.get_mut("dev-dependencies") {
        updated |= update_dependencies_table(dev_dependencies, &mut package_versions)?;
    }

    // Update build-dependencies
    if let Some(Value::Table(build_dependencies)) = doc.get_mut("build-dependencies") {
        updated |= update_dependencies_table(build_dependencies, &mut package_versions)?;
    }

    if updated {
        // Write the updated TOML back to the file
        fs::write(path, doc.to_string())?;
        info!("Updated Cargo.toml with latest versions");
    }

    Ok(updated)
}

/// Update Cargo.toml with cached versions
fn update_cargo_toml_from_cache(
    path: &Path,
    rust_versions: &PackageManagerVersions,
) -> Result<bool> {
    debug!("Updating Cargo.toml from cache at: {}", path.display());

    let content = fs::read_to_string(path)?;
    let mut doc = content
        .parse::<Table>()
        .context("Failed to parse Cargo.toml")?;

    let mut updated = false;

    // Update dependencies
    if let Some(Value::Table(dependencies)) = doc.get_mut("dependencies") {
        updated |= update_dependencies_table_from_cache(dependencies, rust_versions)?;
    }

    // Update dev-dependencies
    if let Some(Value::Table(dev_dependencies)) = doc.get_mut("dev-dependencies") {
        updated |= update_dependencies_table_from_cache(dev_dependencies, rust_versions)?;
    }

    // Update build-dependencies
    if let Some(Value::Table(build_dependencies)) = doc.get_mut("build-dependencies") {
        updated |= update_dependencies_table_from_cache(build_dependencies, rust_versions)?;
    }

    if updated {
        // Write the updated TOML back to the file
        fs::write(path, doc.to_string())?;
        info!("Updated Cargo.toml with cached versions");
    }

    Ok(updated)
}

/// Update a dependencies table
fn update_dependencies_table(
    dependencies: &mut Table,
    package_versions: &mut HashMap<String, String>,
) -> Result<bool> {
    let mut updated = false;
    let mut deps_to_update = Vec::new();

    // First collect all dependencies we need to update
    for (name, value) in dependencies.iter() {
        match value {
            Value::String(version) => {
                // Simple version string
                if !version.starts_with('*') && !version.starts_with("workspace") {
                    deps_to_update.push(name.clone());
                }
            }
            Value::Table(dep_table) => {
                // Dependency table with additional settings
                if let Some(Value::String(version)) = dep_table.get("version") {
                    if !version.starts_with('*') && !version.starts_with("workspace") {
                        deps_to_update.push(name.clone());
                    }
                }
            }
            _ => {}
        }
    }

    // Then update them with the latest versions
    for name in deps_to_update {
        let latest_version = match package_versions.get(&name) {
            Some(version) => version.clone(),
            None => {
                // Fetch latest version from crates.io
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
        match dependencies.get_mut(&name) {
            Some(Value::String(version_mut)) => {
                *version_mut = latest_version.clone();
                updated = true;
            }
            Some(Value::Table(dep_table)) => {
                if let Some(Value::String(version_mut)) = dep_table.get_mut("version") {
                    *version_mut = latest_version;
                    updated = true;
                }
            }
            _ => {}
        }
    }

    Ok(updated)
}

/// Update a dependencies table from cache
fn update_dependencies_table_from_cache(
    dependencies: &mut Table,
    rust_versions: &PackageManagerVersions,
) -> Result<bool> {
    let mut updated = false;

    for (name, value) in dependencies.iter_mut() {
        // Check if we have a cached version for this package
        if let Some(cached_version) = rust_versions.packages.get(name) {
            match value {
                Value::String(version) => {
                    // Simple version string
                    if !version.starts_with('*') && !version.starts_with("workspace") {
                        *version = cached_version.clone();
                        updated = true;
                    }
                }
                Value::Table(dep_table) => {
                    // Dependency table with additional settings
                    if let Some(Value::String(version)) = dep_table.get_mut("version") {
                        if !version.starts_with('*') && !version.starts_with("workspace") {
                            *version = cached_version.clone();
                            updated = true;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(updated)
}

/// Get the latest version of a crate from crates.io
fn get_latest_version(crate_name: &str) -> Result<String> {
    debug!("Getting latest version for crate: {}", crate_name);

    let client = Client::new();
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);

    let response = client.get(&url).header("User-Agent", "jig-tool").send()?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to get crate info: HTTP {}",
            response.status()
        ));
    }

    let crate_data: CratesIoResponse = response.json()?;

    // Find the latest non-yanked version
    let latest_version = crate_data
        .versions
        .iter()
        .find(|v| !v.yanked)
        .ok_or_else(|| anyhow!("No non-yanked versions found"))?;

    debug!("Latest version of {} is {}", crate_name, latest_version.num);

    Ok(latest_version.num.clone())
}

/// Find workspace members in Cargo.toml
fn find_workspace_members(repo_path: &Path, cargo_toml_path: &Path) -> Result<Vec<PathBuf>> {
    debug!(
        "Finding workspace members in: {}",
        cargo_toml_path.display()
    );

    let content = fs::read_to_string(cargo_toml_path)?;
    let doc = content
        .parse::<Table>()
        .context("Failed to parse Cargo.toml")?;

    let mut member_paths = Vec::new();

    if let Some(Value::Table(workspace)) = doc.get("workspace") {
        if let Some(Value::Array(members)) = workspace.get("members") {
            for member in members {
                if let Value::String(member_str) = member {
                    // Handle glob patterns in workspace members
                    if member_str.contains('*') {
                        let glob_pattern =
                            format!("{}/{}/Cargo.toml", repo_path.display(), member_str);
                        member_paths.extend(glob::glob(&glob_pattern)?.flatten());
                    } else {
                        let member_cargo_path = repo_path.join(member_str).join("Cargo.toml");
                        if member_cargo_path.exists() {
                            member_paths.push(member_cargo_path);
                        }
                    }
                }
            }
        }
    }

    Ok(member_paths)
}

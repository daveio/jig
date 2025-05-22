use super::PackageManagerResult;
use crate::config::{PackageManagerVersions, VersionsConfig};
use anyhow::{anyhow, Context, Result};
use log::{debug, info, warn};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct NpmPackageInfo {
    #[serde(rename = "dist-tags")]
    dist_tags: HashMap<String, String>,
}

/// Bump JavaScript/TypeScript package versions
pub fn bump_versions(repo_path: &Path) -> Result<PackageManagerResult> {
    debug!(
        "Checking for JavaScript/TypeScript package managers in: {}",
        repo_path.display()
    );

    let mut result = PackageManagerResult {
        name: "JavaScript/TypeScript".to_string(),
        updated_files: Vec::new(),
        errors: Vec::new(),
    };

    // Check for package.json
    let package_json_path = repo_path.join("package.json");
    if package_json_path.exists() {
        debug!("Found package.json");

        match update_package_json(&package_json_path) {
            Ok(updated) => {
                if updated {
                    result.updated_files.push(package_json_path);
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to update package.json: {}", e));
            }
        }
    }

    Ok(result)
}

/// Bump JavaScript/TypeScript package versions using cached version information
pub fn bump_versions_from_cache(
    repo_path: &Path,
    versions_config: &VersionsConfig,
) -> Result<PackageManagerResult> {
    debug!(
        "Checking for JavaScript/TypeScript package managers using cached versions in: {}",
        repo_path.display()
    );

    let mut result = PackageManagerResult {
        name: "JavaScript/TypeScript".to_string(),
        updated_files: Vec::new(),
        errors: Vec::new(),
    };

    // Check if we have cached JavaScript/TypeScript versions
    if !versions_config.package_managers.contains_key("javascript")
        && !versions_config.package_managers.contains_key("typescript")
    {
        debug!("No cached JavaScript/TypeScript versions found");
        return Ok(result);
    }

    // Try to find javascript or typescript versions
    let js_versions = versions_config
        .package_managers
        .get("javascript")
        .or_else(|| versions_config.package_managers.get("typescript"));

    let js_versions = match js_versions {
        Some(versions) => versions,
        None => {
            debug!("No cached JavaScript/TypeScript versions found");
            return Ok(result);
        }
    };

    // Check for package.json
    let package_json_path = repo_path.join("package.json");
    if package_json_path.exists() {
        debug!("Found package.json");

        match update_package_json_from_cache(&package_json_path, js_versions) {
            Ok(updated) => {
                if updated {
                    result.updated_files.push(package_json_path);
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to update package.json from cache: {}", e));
            }
        }
    }

    Ok(result)
}

/// Update package.json with latest versions
fn update_package_json(path: &Path) -> Result<bool> {
    debug!("Updating package.json at: {}", path.display());

    let content = fs::read_to_string(path)?;
    let mut doc: Value = serde_json::from_str(&content).context("Failed to parse package.json")?;

    let mut updated = false;
    let mut package_versions = HashMap::new();

    // Update dependencies
    if let Some(deps) = doc.get_mut("dependencies").and_then(|d| d.as_object_mut()) {
        updated |= update_dependencies_object(deps, &mut package_versions)?;
    }

    // Update devDependencies
    if let Some(deps) = doc
        .get_mut("devDependencies")
        .and_then(|d| d.as_object_mut())
    {
        updated |= update_dependencies_object(deps, &mut package_versions)?;
    }

    // Update peerDependencies
    if let Some(deps) = doc
        .get_mut("peerDependencies")
        .and_then(|d| d.as_object_mut())
    {
        updated |= update_dependencies_object(deps, &mut package_versions)?;
    }

    if updated {
        // Write the updated JSON back to the file
        fs::write(path, serde_json::to_string_pretty(&doc)?)?;
        info!("Updated package.json with latest versions");
    }

    Ok(updated)
}

/// Update package.json with cached versions
fn update_package_json_from_cache(
    path: &Path,
    js_versions: &PackageManagerVersions,
) -> Result<bool> {
    debug!("Updating package.json from cache at: {}", path.display());

    let content = fs::read_to_string(path)?;
    let mut doc: Value = serde_json::from_str(&content).context("Failed to parse package.json")?;

    let mut updated = false;

    // Update dependencies
    if let Some(deps) = doc.get_mut("dependencies").and_then(|d| d.as_object_mut()) {
        updated |= update_dependencies_object_from_cache(deps, js_versions)?;
    }

    // Update devDependencies
    if let Some(deps) = doc
        .get_mut("devDependencies")
        .and_then(|d| d.as_object_mut())
    {
        updated |= update_dependencies_object_from_cache(deps, js_versions)?;
    }

    // Update peerDependencies
    if let Some(deps) = doc
        .get_mut("peerDependencies")
        .and_then(|d| d.as_object_mut())
    {
        updated |= update_dependencies_object_from_cache(deps, js_versions)?;
    }

    if updated {
        // Write the updated JSON back to the file
        fs::write(path, serde_json::to_string_pretty(&doc)?)?;
        info!("Updated package.json with cached versions");
    }

    Ok(updated)
}

/// Update dependencies in a JSON object
fn update_dependencies_object(
    deps: &mut serde_json::Map<String, Value>,
    package_versions: &mut HashMap<String, String>,
) -> Result<bool> {
    let mut updated = false;
    let mut deps_to_update = Vec::new();

    // First collect all dependencies we need to update
    for (name, version) in deps.iter() {
        if let Value::String(version_str) = version {
            if version_str.starts_with('^')
                || version_str.starts_with('~')
                || version_str.starts_with('=')
            {
                deps_to_update.push(name.clone());
            }
        }
    }

    // Then update them with the latest versions
    for name in deps_to_update {
        let latest_version = match package_versions.get(&name) {
            Some(version) => version.clone(),
            None => {
                // Fetch latest version from npm registry
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
        if let Some(version_value) = deps.get_mut(&name) {
            *version_value = json!(latest_version);
            updated = true;
        }
    }

    Ok(updated)
}

/// Update dependencies in a JSON object from cache
fn update_dependencies_object_from_cache(
    deps: &mut serde_json::Map<String, Value>,
    js_versions: &PackageManagerVersions,
) -> Result<bool> {
    let mut updated = false;

    for (name, version_value) in deps.iter_mut() {
        // Check if we have a cached version for this package
        if let Some(cached_version) = js_versions.packages.get(name) {
            if let Value::String(version_str) = version_value {
                if version_str.starts_with('^')
                    || version_str.starts_with('~')
                    || version_str.starts_with('=')
                {
                    *version_value = json!(cached_version);
                    updated = true;
                }
            }
        }
    }

    Ok(updated)
}

/// Get the latest version of a package from npm registry
fn get_latest_version(package_name: &str) -> Result<String> {
    debug!("Getting latest version for npm package: {}", package_name);

    let client = Client::new();
    let url = format!("https://registry.npmjs.org/{}", package_name);

    let response = client.get(&url).header("User-Agent", "jig-tool").send()?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to get package info: HTTP {}",
            response.status()
        ));
    }

    let package_data: NpmPackageInfo = response.json()?;

    // Get the 'latest' tag
    if let Some(latest) = package_data.dist_tags.get("latest") {
        debug!("Latest version of {} is {}", package_name, latest);
        Ok(latest.clone())
    } else {
        Err(anyhow!("No 'latest' tag found for package"))
    }
}

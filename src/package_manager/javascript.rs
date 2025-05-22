use super::PackageManagerResult;
use anyhow::{Context, Result};
use log::{debug, info};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

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

/// Update package.json with latest versions
fn update_package_json(path: &Path) -> Result<bool> {
    debug!("Updating package.json at: {}", path.display());

    let content = fs::read_to_string(path)?;
    let mut doc: Value = serde_json::from_str(&content).context("Failed to parse package.json")?;

    let mut updated = false;

    // Update dependencies
    if let Some(deps) = doc.get_mut("dependencies").and_then(|d| d.as_object_mut()) {
        for (_, version) in deps.iter_mut() {
            if let Value::String(v) = version {
                if v.starts_with('^') || v.starts_with('~') || v.starts_with('=') {
                    // Replace version specifiers with "*" to get latest
                    *version = json!("*");
                    updated = true;
                }
            }
        }
    }

    // Update devDependencies
    if let Some(deps) = doc
        .get_mut("devDependencies")
        .and_then(|d| d.as_object_mut())
    {
        for (_, version) in deps.iter_mut() {
            if let Value::String(v) = version {
                if v.starts_with('^') || v.starts_with('~') || v.starts_with('=') {
                    // Replace version specifiers with "*" to get latest
                    *version = json!("*");
                    updated = true;
                }
            }
        }
    }

    // Update peerDependencies
    if let Some(deps) = doc
        .get_mut("peerDependencies")
        .and_then(|d| d.as_object_mut())
    {
        for (_, version) in deps.iter_mut() {
            if let Value::String(v) = version {
                if v.starts_with('^') || v.starts_with('~') || v.starts_with('=') {
                    // Replace version specifiers with "*" to get latest
                    *version = json!("*");
                    updated = true;
                }
            }
        }
    }

    if updated {
        // Write the updated JSON back to the file
        fs::write(path, serde_json::to_string_pretty(&doc)?)?;
        info!("Updated package.json with latest versions");
    }

    Ok(updated)
}

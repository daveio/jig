use super::PackageManagerResult;
use anyhow::{Context, Result};
use log::{debug, info};
use std::fs;
use std::path::{Path, PathBuf};
use toml::{Table, Value};

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

/// Update Cargo.toml with latest versions
fn update_cargo_toml(path: &Path) -> Result<bool> {
    debug!("Updating Cargo.toml at: {}", path.display());

    let content = fs::read_to_string(path)?;
    let mut doc = content
        .parse::<Table>()
        .context("Failed to parse Cargo.toml")?;

    let mut updated = false;

    // Update dependencies
    if let Some(Value::Table(dependencies)) = doc.get_mut("dependencies") {
        updated |= update_dependencies_table(dependencies);
    }

    // Update dev-dependencies
    if let Some(Value::Table(dev_dependencies)) = doc.get_mut("dev-dependencies") {
        updated |= update_dependencies_table(dev_dependencies);
    }

    // Update build-dependencies
    if let Some(Value::Table(build_dependencies)) = doc.get_mut("build-dependencies") {
        updated |= update_dependencies_table(build_dependencies);
    }

    if updated {
        // Write the updated TOML back to the file
        fs::write(path, doc.to_string())?;
        info!("Updated Cargo.toml with latest versions");
    }

    Ok(updated)
}

/// Update a dependencies table
fn update_dependencies_table(dependencies: &mut Table) -> bool {
    let mut updated = false;

    for (_, value) in dependencies.iter_mut() {
        match value {
            Value::String(version) => {
                // Simple version string, replace with "*"
                if !version.starts_with('*') {
                    *version = "*".to_string();
                    updated = true;
                }
            }
            Value::Table(dep_table) => {
                // Dependency table with additional settings
                if let Some(Value::String(version)) = dep_table.get_mut("version") {
                    if !version.starts_with('*') {
                        *version = "*".to_string();
                        updated = true;
                    }
                }
            }
            _ => {}
        }
    }

    updated
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

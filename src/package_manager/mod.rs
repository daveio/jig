use crate::config::VersionsConfig;
use anyhow::Result;
use git2::Repository;
use log::{debug, info};
use std::path::{Path, PathBuf};

pub mod github_actions;
pub mod javascript;
pub mod python;
pub mod rust;

/// Result of a package manager version bump
pub struct PackageManagerResult {
    pub name: String,
    pub updated_files: Vec<PathBuf>,
    pub errors: Vec<String>,
}

/// Result of GitHub Actions update
pub struct GitHubActionsResult {
    pub updated: bool,
    pub workflows: Vec<PathBuf>,
    pub errors: Vec<String>,
}

/// Bump all versions in a repository
pub fn bump_all_versions(repo_path: &Path) -> Result<Vec<PackageManagerResult>> {
    info!(
        "Bumping all versions in repository at: {}",
        repo_path.display()
    );

    let mut results = Vec::new();

    // Try to bump Python dependencies
    match python::bump_versions(repo_path) {
        Ok(result) => {
            if !result.updated_files.is_empty() {
                results.push(result);
            }
        }
        Err(e) => debug!("Failed to bump Python dependencies: {}", e),
    }

    // Try to bump JavaScript/TypeScript dependencies
    match javascript::bump_versions(repo_path) {
        Ok(result) => {
            if !result.updated_files.is_empty() {
                results.push(result);
            }
        }
        Err(e) => debug!("Failed to bump JavaScript dependencies: {}", e),
    }

    // Try to bump Rust dependencies
    match rust::bump_versions(repo_path) {
        Ok(result) => {
            if !result.updated_files.is_empty() {
                results.push(result);
            }
        }
        Err(e) => debug!("Failed to bump Rust dependencies: {}", e),
    }

    Ok(results)
}

/// Bump all versions in a repository using cached version information
pub fn bump_all_versions_from_cache(
    repo_path: &Path,
    versions_config: &VersionsConfig,
) -> Result<Vec<PackageManagerResult>> {
    info!(
        "Bumping all versions from cache in repository at: {}",
        repo_path.display()
    );

    let mut results = Vec::new();

    // Try to bump Python dependencies using cache
    if versions_config.package_managers.contains_key("python") {
        match python::bump_versions_from_cache(repo_path, versions_config) {
            Ok(result) => {
                if !result.updated_files.is_empty() {
                    results.push(result);
                }
            }
            Err(e) => debug!("Failed to bump Python dependencies from cache: {}", e),
        }
    }

    // Try to bump JavaScript/TypeScript dependencies using cache
    if versions_config.package_managers.contains_key("javascript")
        || versions_config.package_managers.contains_key("typescript")
    {
        match javascript::bump_versions_from_cache(repo_path, versions_config) {
            Ok(result) => {
                if !result.updated_files.is_empty() {
                    results.push(result);
                }
            }
            Err(e) => debug!("Failed to bump JavaScript dependencies from cache: {}", e),
        }
    }

    // Try to bump Rust dependencies using cache
    if versions_config.package_managers.contains_key("rust") {
        match rust::bump_versions_from_cache(repo_path, versions_config) {
            Ok(result) => {
                if !result.updated_files.is_empty() {
                    results.push(result);
                }
            }
            Err(e) => debug!("Failed to bump Rust dependencies from cache: {}", e),
        }
    }

    Ok(results)
}

/// Update GitHub Actions workflows
pub fn update_github_actions(repo_path: &Path, repo: &Repository) -> Result<GitHubActionsResult> {
    debug!(
        "Updating GitHub Actions workflows in: {}",
        repo_path.display()
    );

    github_actions::update_workflows(repo_path, repo)
}

/// Update GitHub Actions workflows using cached version information
pub fn update_github_actions_from_cache(
    repo_path: &Path,
    repo: &Repository,
    versions_config: &VersionsConfig,
) -> Result<GitHubActionsResult> {
    debug!(
        "Updating GitHub Actions workflows from cache in: {}",
        repo_path.display()
    );

    // Check if we have cached GitHub Actions versions
    if versions_config.github_actions.is_empty() {
        debug!("No cached GitHub Actions versions found");
        return Ok(GitHubActionsResult {
            updated: false,
            workflows: Vec::new(),
            errors: Vec::new(),
        });
    }

    github_actions::update_workflows_from_cache(repo_path, repo, versions_config)
}

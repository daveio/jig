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

/// Update GitHub Actions workflows
pub fn update_github_actions(repo_path: &Path, repo: &Repository) -> Result<GitHubActionsResult> {
    debug!(
        "Updating GitHub Actions workflows in: {}",
        repo_path.display()
    );

    github_actions::update_workflows(repo_path, repo)
}

use super::GitHubActionsResult;
use crate::git;
use anyhow::{Context, Result};
use git2::Repository;
use log::{debug, info};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Update GitHub Actions workflows to use the latest commit
pub fn update_workflows(repo_path: &Path, repo: &Repository) -> Result<GitHubActionsResult> {
    debug!(
        "Updating GitHub Actions workflows in: {}",
        repo_path.display()
    );

    let mut result = GitHubActionsResult {
        updated: false,
        workflows: Vec::new(),
        errors: Vec::new(),
    };

    // Check if .github/workflows directory exists
    let workflows_dir = repo_path.join(".github/workflows");
    if !workflows_dir.exists() {
        debug!("No .github/workflows directory found");
        return Ok(result);
    }

    // Get the default branch name
    let default_branch = match git::get_default_branch(repo) {
        Ok(branch) => branch,
        Err(e) => {
            result
                .errors
                .push(format!("Failed to get default branch: {}", e));
            return Ok(result);
        }
    };

    // Find the latest commit on the default branch
    let latest_commit = get_latest_commit_for_branch(repo, &default_branch)?;

    debug!(
        "Latest commit on {} branch: {}",
        default_branch, latest_commit
    );

    // Walk through the workflows directory
    for entry in WalkDir::new(&workflows_dir) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                result
                    .errors
                    .push(format!("Error walking workflows directory: {}", e));
                continue;
            }
        };

        let path = entry.path();

        // Skip directories and non-YAML files
        if !path.is_file()
            || path
                .extension()
                .is_none_or(|ext| ext != "yml" && ext != "yaml")
        {
            continue;
        }

        debug!("Found workflow file: {}", path.display());

        // Update the workflow file
        match update_workflow_file(path, &default_branch, &latest_commit) {
            Ok(updated) => {
                if updated {
                    result.workflows.push(path.to_path_buf());
                    result.updated = true;
                }
            }
            Err(e) => {
                result.errors.push(format!(
                    "Failed to update workflow file {}: {}",
                    path.display(),
                    e
                ));
            }
        }
    }

    Ok(result)
}

/// Update a GitHub Actions workflow file
fn update_workflow_file(path: &Path, branch: &str, commit: &str) -> Result<bool> {
    debug!("Updating workflow file: {}", path.display());

    let content = fs::read_to_string(path)?;

    // For this use case, we're just doing regex replacements on the content
    // No need to parse the YAML since we're just updating action references
    let mut updated_content = content.clone();
    let mut updated = false;

    // Patterns to look for in workflow files
    let patterns = [
        // uses: actions/xxx@main or @master
        (
            format!(r#"uses: ([^@]+)@({}|master)"#, branch),
            format!(r#"uses: $1@{}"#, commit),
        ),
        // uses: actions/xxx@v1, v2, etc.
        (
            r#"uses: ([^@]+)@v\d+"#.to_string(),
            format!(r#"uses: $1@{}"#, commit),
        ),
        // ref: 'refs/heads/main' or 'refs/heads/master'
        (
            format!(r#"ref: 'refs/heads/({}|master)'"#, branch),
            format!(r#"ref: 'refs/heads/{}'"#, branch),
        ),
    ];

    // Apply each pattern
    for (pattern, replacement) in patterns {
        let regex = match regex::Regex::new(&pattern) {
            Ok(re) => re,
            Err(e) => {
                debug!("Failed to compile regex {}: {}", pattern, e);
                continue;
            }
        };

        if regex.is_match(&updated_content) {
            updated_content = regex
                .replace_all(&updated_content, replacement.as_str())
                .to_string();
            updated = true;
        }
    }

    if updated {
        fs::write(path, updated_content)?;
        info!("Updated workflow file: {}", path.display());
    }

    Ok(updated)
}

/// Get the latest commit hash for a branch
fn get_latest_commit_for_branch(repo: &Repository, branch_name: &str) -> Result<String> {
    debug!("Getting latest commit for branch: {}", branch_name);

    let branch = repo
        .find_branch(branch_name, git2::BranchType::Local)
        .context(format!("Failed to find branch {}", branch_name))?;

    let reference = branch.into_reference();
    let commit = reference
        .peel_to_commit()
        .context("Failed to peel reference to commit")?;

    Ok(commit.id().to_string())
}

use super::GitHubActionsResult;
use crate::config::VersionsConfig;
use crate::git;
use anyhow::{anyhow, Result};
use git2::Repository;
use log::{debug, info, warn};
use octocrab::Octocrab;
use regex::Regex;
use reqwest::blocking::Client;
use std::fs;
use std::path::Path;
use tokio::runtime::Runtime;
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
    let _default_branch = match git::get_default_branch(repo) {
        Ok(branch) => branch,
        Err(e) => {
            result
                .errors
                .push(format!("Failed to get default branch: {}", e));
            return Ok(result);
        }
    };

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
        match update_workflow_file(path) {
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

/// Update GitHub Actions workflows using cached versions
pub fn update_workflows_from_cache(
    repo_path: &Path,
    _repo: &Repository,
    versions_config: &VersionsConfig,
) -> Result<GitHubActionsResult> {
    debug!(
        "Updating GitHub Actions workflows from cache in: {}",
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

        // Update the workflow file using cached versions
        match update_workflow_file_from_cache(path, versions_config) {
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
fn update_workflow_file(path: &Path) -> Result<bool> {
    debug!("Updating workflow file: {}", path.display());

    let content = fs::read_to_string(path)?;
    let mut updated_content = content.clone();
    let mut updated = false;

    // Find all action references in the workflow file
    let uses_regex = Regex::new(r#"uses:\s+([^@]+)@([^\s]+)"#)?;
    let ref_regex = Regex::new(r#"ref:\s+'refs/heads/([^']+)'"#)?;

    // Collect action references that need to be updated
    let mut actions_to_update = Vec::new();

    // Find all 'uses:' statements
    for captures in uses_regex.captures_iter(&content) {
        if let (Some(action), Some(version)) = (captures.get(1), captures.get(2)) {
            let action_name = action.as_str().trim();
            let version_str = version.as_str();

            // Skip if already using a commit hash (40 hex characters)
            if version_str.len() == 40 && version_str.chars().all(|c| c.is_ascii_hexdigit()) {
                continue;
            }

            // Skip if using a version tag (like v1, v2, etc.) and the action is not a GitHub-hosted action
            // This is a simple heuristic; we could make this more sophisticated
            if version_str.starts_with('v') && !action_name.starts_with("actions/") {
                continue;
            }

            // Add to the list of actions to update
            actions_to_update.push(action_name.to_string());
        }
    }

    // Update each action with the latest commit
    for action in actions_to_update {
        match get_latest_commit_for_action(&action) {
            Ok(commit) => {
                debug!("Found latest commit for {}: {}", action, commit);

                // Replace the action version with the commit hash
                let pattern = format!(r#"uses:\s+{}@[^\s]+"#, regex::escape(&action));
                let replacement = format!("uses: {}@{}", action, commit);

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
            Err(e) => {
                debug!("Failed to get latest commit for {}: {}", action, e);
            }
        }
    }

    // Find and update branch references
    for captures in ref_regex.captures_iter(&content) {
        if let Some(branch) = captures.get(1) {
            let branch_name = branch.as_str();

            // Only update 'main' or 'master' references for now
            if branch_name == "main" || branch_name == "master" {
                // For simplicity, we'll standardize on 'main'
                let pattern = format!(r#"ref:\s+'refs/heads/{}'"#, branch_name);
                let replacement = "ref: 'refs/heads/main'";

                let regex = match regex::Regex::new(&pattern) {
                    Ok(re) => re,
                    Err(e) => {
                        debug!("Failed to compile regex {}: {}", pattern, e);
                        continue;
                    }
                };

                if regex.is_match(&updated_content) && branch_name != "main" {
                    updated_content = regex.replace_all(&updated_content, replacement).to_string();
                    updated = true;
                }
            }
        }
    }

    if updated {
        fs::write(path, updated_content)?;
        info!("Updated workflow file: {}", path.display());
    }

    Ok(updated)
}

/// Update workflow file using cached versions
fn update_workflow_file_from_cache(path: &Path, versions_config: &VersionsConfig) -> Result<bool> {
    debug!("Updating workflow file from cache: {}", path.display());

    let content = fs::read_to_string(path)?;
    let mut updated_content = content.clone();
    let mut updated = false;

    // Find all action references in the workflow file
    let uses_regex = Regex::new(r#"uses:\s+([^@]+)@([^\s]+)"#)?;

    // Collect action references that need to be updated
    let mut actions_to_update = Vec::new();

    // Find all 'uses:' statements
    for captures in uses_regex.captures_iter(&content) {
        if let (Some(action), Some(version)) = (captures.get(1), captures.get(2)) {
            let action_name = action.as_str().trim();
            let version_str = version.as_str();

            // Skip if already using a commit hash (40 hex characters)
            if version_str.len() == 40 && version_str.chars().all(|c| c.is_ascii_hexdigit()) {
                continue;
            }

            // Check if we have a cached version for this action
            if let Some(commit) = versions_config.github_actions.get(action_name) {
                actions_to_update.push((action_name.to_string(), commit.clone()));
            }
        }
    }

    // Update each action with the cached commit
    for (action, commit) in actions_to_update {
        debug!("Using cached commit for {}: {}", action, commit);

        // Replace the action version with the commit hash
        let pattern = format!(r#"uses:\s+{}@[^\s]+"#, regex::escape(&action));
        let replacement = format!("uses: {}@{}", action, commit);

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
        info!("Updated workflow file from cache: {}", path.display());
    }

    Ok(updated)
}

/// Get the latest commit hash for a GitHub action
fn get_latest_commit_for_action(action: &str) -> Result<String> {
    debug!("Getting latest commit for action: {}", action);

    // Parse the action name to extract owner, repo, and potentially path
    let (owner, repo) = parse_action_name(action)?;

    // Try to get the latest commit using Octocrab
    match get_latest_commit_octocrab(&owner, &repo) {
        Ok(commit) => Ok(commit),
        Err(e) => {
            warn!("Failed to get commit using Octocrab: {}", e);
            // Fall back to the GitHub CLI if available
            match get_latest_commit_gh_cli(&owner, &repo) {
                Ok(commit) => Ok(commit),
                Err(e) => {
                    warn!("Failed to get commit using GitHub CLI: {}", e);
                    // Fall back to direct API call
                    get_latest_commit_direct_api(&owner, &repo)
                }
            }
        }
    }
}

/// Parse a GitHub action name into owner and repo components
fn parse_action_name(action: &str) -> Result<(String, String)> {
    // Handle formats like:
    // - actions/checkout
    // - github/codeql-action/upload-sarif (repo is github/codeql-action)
    // - ljharb/actions/.github/workflows/node.yml (repo is ljharb/actions)

    let parts: Vec<&str> = action.trim().split('/').collect();
    if parts.len() < 2 {
        return Err(anyhow!("Invalid action name format: {}", action));
    }

    // The first part is always the owner
    let owner = parts[0].to_string();

    // For the repo, we need to handle special cases
    let repo = if parts.len() >= 3 {
        // For cases like github/codeql-action/upload-sarif or ljharb/actions/.github/workflows/node.yml
        // Check if there's a file extension in later parts to distinguish file specs
        if parts.iter().skip(2).any(|part| part.contains('.')) {
            // This looks like a file specification like ljharb/actions/.github/workflows/node.yml
            // The repo is just owner/repo (first two parts)
            parts[1].to_string()
        } else {
            // This looks like an action subdirectory like github/codeql-action/upload-sarif
            // The repo is still just the second part
            parts[1].to_string()
        }
    } else {
        // Standard case: owner/repo
        parts[1].to_string()
    };

    debug!(
        "Parsed action '{}' as owner='{}', repo='{}'",
        action, owner, repo
    );
    Ok((owner, repo))
}

/// Get the latest commit using Octocrab
fn get_latest_commit_octocrab(owner: &str, repo: &str) -> Result<String> {
    debug!(
        "Getting latest commit using Octocrab for {}/{}",
        owner, repo
    );

    // Create a tokio runtime for the async GitHub API calls
    let rt = Runtime::new()?;

    rt.block_on(async {
        // Create an unauthenticated GitHub client
        let octocrab = Octocrab::builder().build()?;

        // Try 'main' branch first
        match octocrab.repos(owner, repo).get().await {
            Ok(repository) => {
                if let Some(default_branch) = repository.default_branch {
                    // Get the latest commit on the default branch
                    let commits = octocrab
                        .repos(owner, repo)
                        .list_commits()
                        .branch(&default_branch)
                        .per_page(1)
                        .send()
                        .await?;

                    if let Some(commit) = commits.items.into_iter().next() {
                        Ok(commit.sha)
                    } else {
                        Err(anyhow!("No commits found on default branch"))
                    }
                } else {
                    Err(anyhow!("No default branch found"))
                }
            }
            Err(e) => Err(anyhow!("Failed to get repository info: {}", e)),
        }
    })
}

/// Get the latest commit using the GitHub CLI
fn get_latest_commit_gh_cli(owner: &str, repo: &str) -> Result<String> {
    debug!(
        "Getting latest commit using GitHub CLI for {}/{}",
        owner, repo
    );

    // Try 'main' branch first
    let output = std::process::Command::new("gh")
        .args([
            "api",
            &format!("repos/{}/{}/commits/main", owner, repo),
            "--jq",
            ".sha",
        ])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(commit)
        }
        _ => {
            // Fall back to 'master' branch
            let output = std::process::Command::new("gh")
                .args([
                    "api",
                    &format!("repos/{}/{}/commits/master", owner, repo),
                    "--jq",
                    ".sha",
                ])
                .output()?;

            if output.status.success() {
                let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(commit)
            } else {
                Err(anyhow!("Failed to get commit using GitHub CLI"))
            }
        }
    }
}

/// Get the latest commit using a direct API call
fn get_latest_commit_direct_api(owner: &str, repo: &str) -> Result<String> {
    debug!(
        "Getting latest commit using direct API for {}/{}",
        owner, repo
    );

    let client = Client::new();

    // Try 'main' branch first
    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/main",
        owner, repo
    );
    let response = client.get(&url).header("User-Agent", "jig-tool").send();

    match response {
        Ok(response) if response.status().is_success() => {
            let json: serde_json::Value = response.json()?;
            if let Some(sha) = json.get("sha").and_then(|s| s.as_str()) {
                Ok(sha.to_string())
            } else {
                Err(anyhow!("SHA not found in response"))
            }
        }
        _ => {
            // Fall back to 'master' branch
            let url = format!(
                "https://api.github.com/repos/{}/{}/commits/master",
                owner, repo
            );
            let response = client.get(&url).header("User-Agent", "jig-tool").send()?;

            if response.status().is_success() {
                let json: serde_json::Value = response.json()?;
                if let Some(sha) = json.get("sha").and_then(|s| s.as_str()) {
                    Ok(sha.to_string())
                } else {
                    Err(anyhow!("SHA not found in response"))
                }
            } else {
                Err(anyhow!("Failed to get commit using direct API"))
            }
        }
    }
}

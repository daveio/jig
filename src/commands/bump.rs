use crate::cli::BumpArgs;
use crate::git;
use crate::package_manager;
use crate::utils::paths;
use anyhow::{Context, Result};
use log::{info, debug};

/// Execute the 'bump' command
pub fn execute(args: &BumpArgs, dry_run: bool) -> Result<()> {
    // Determine the repository path
    let repo_path = match &args.repository {
        Some(path) => paths::to_absolute_path(path)?,
        None => paths::get_current_dir()?,
    };

    info!("Bumping versions in repository at: {}", repo_path.display());

    if dry_run {
        info!("Dry run mode: No changes will be made");
        info!("Would bump versions in repository at {}", repo_path.display());
        return Ok(());
    }

    // Get the repository
    let repo = git::open_repository(&repo_path)
        .context("Failed to open git repository")?;

    // Detect and update package managers
    let result = package_manager::bump_all_versions(&repo_path)?;

    if result.is_empty() {
        info!("No package managers found or no updates needed.");
        return Ok(());
    }

    debug!("Updated {} package managers", result.len());

    for pm in &result {
        info!("Updated {} dependencies", pm.name);
    }

    // Update GitHub Actions if present
    let github_actions_result = package_manager::update_github_actions(&repo_path, &repo)?;

    if github_actions_result.updated {
        info!("Updated {} GitHub Actions workflows", github_actions_result.workflows.len());
    }

    // Commit the changes
    git::commit_all(&repo, "Bump dependencies to latest versions")
        .context("Failed to commit version updates")?;

    info!("Version bumping completed successfully");

    Ok(())
}

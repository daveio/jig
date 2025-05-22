use crate::cli::BumpArgs;
use crate::config::{ConfigManager, PackageManagerVersions};
use crate::git;
use crate::package_manager;
use crate::utils::paths;
use anyhow::{Context, Result};
use chrono::Utc;
use log::{debug, info};
use std::collections::HashMap;

/// Execute the 'bump' command
pub fn execute(args: &BumpArgs, dry_run: bool) -> Result<()> {
    // Determine the repository path
    let repo_path = match &args.repository {
        Some(path) => paths::to_absolute_path(path)?,
        None => paths::get_current_dir()?,
    };

    // Load configuration manager to access and update versions
    let mut config_manager = ConfigManager::new()?;
    let versions_config = config_manager.versions_config_mut();

    if args.cached {
        info!("Using cached versions from: {}", repo_path.display());

        if versions_config.last_checked.is_none() {
            info!("No cached versions available. Run without --cached first.");
            return Ok(());
        }

        info!(
            "Last checked: {}",
            versions_config.last_checked.as_ref().unwrap()
        );
    } else {
        info!("Bumping versions in repository at: {}", repo_path.display());
    }

    if dry_run {
        info!("Dry run mode: No changes will be made");
        info!(
            "Would bump versions in repository at {}",
            repo_path.display()
        );
        return Ok(());
    }

    // Get the repository
    let repo = git::open_repository(&repo_path).context("Failed to open git repository")?;

    // Detect and update package managers - either using cached versions or checking for updates
    let result = if args.cached {
        // Use cached versions instead of checking online
        package_manager::bump_all_versions_from_cache(&repo_path, versions_config)?
    } else {
        // Check for updates online and update the cache
        let result = package_manager::bump_all_versions(&repo_path)?;

        // Update the versions cache if we found any updates
        if !result.is_empty() {
            // Record the timestamp
            versions_config.last_checked = Some(Utc::now().to_rfc3339());

            // Update package manager versions
            for pm in &result {
                let pm_name = pm.name.clone();

                // Create entry if it doesn't exist
                if !versions_config.package_managers.contains_key(&pm_name) {
                    versions_config.package_managers.insert(
                        pm_name.clone(),
                        PackageManagerVersions {
                            packages: HashMap::new(),
                        },
                    );
                }

                // TODO: Extract and store actual package versions here
                // This would depend on having access to the specific version information
                // which would need to be added to the package_manager module
            }

            // Save the updated versions cache
            config_manager.save_versions()?;
        }

        result
    };

    if result.is_empty() {
        info!("No package managers found or no updates needed.");
        return Ok(());
    }

    debug!("Updated {} package managers", result.len());

    for pm in &result {
        info!("Updated {} dependencies", pm.name);
    }

    // Update GitHub Actions if present
    let github_actions_result = if args.cached {
        // Use cached GitHub Actions versions
        package_manager::update_github_actions_from_cache(&repo_path, &repo, versions_config)?
    } else {
        // Check for updates online
        let result = package_manager::update_github_actions(&repo_path, &repo)?;

        // Update the cache if we found any updates
        if result.updated {
            // TODO: Extract and store actual GitHub Actions versions
            // This would depend on having access to the specific version information
        }

        result
    };

    if github_actions_result.updated {
        info!(
            "Updated {} GitHub Actions workflows",
            github_actions_result.workflows.len()
        );
    }

    // Commit the changes
    git::commit_all(&repo, "Bump dependencies to latest versions")
        .context("Failed to commit version updates")?;

    info!("Version bumping completed successfully");

    Ok(())
}

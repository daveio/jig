use crate::cli::{BumpArgs, BumpEcosystem, CommonOptions};
use crate::config::{ConfigManager, PackageManagerVersions};
use crate::git;
use crate::package_manager;
use crate::utils::paths;
use anyhow::{Context, Result};
use chrono::Utc;
use log::{debug, info};
use std::collections::HashMap;

/// Get the display name for an ecosystem
fn ecosystem_name(ecosystem: &BumpEcosystem) -> &'static str {
    match ecosystem {
        BumpEcosystem::Node => "Node.js",
        BumpEcosystem::Python => "Python",
        BumpEcosystem::Ruby => "Ruby",
        BumpEcosystem::Rust => "Rust",
        BumpEcosystem::Java => "Java",
        BumpEcosystem::Go => "Go",
        BumpEcosystem::Actions => "GitHub Actions",
    }
}

/// Execute the 'bump' command
pub fn execute(args: &BumpArgs, options: &CommonOptions) -> Result<()> {
    // Determine the repository path
    let repo_path = match &args.repository {
        Some(path) => paths::to_absolute_path(path)?,
        None => paths::get_current_dir()?,
    };

    // Check for ecosystem-specific targeting
    let ecosystem_filter = &args.ecosystem;

    if options.verbose {
        match ecosystem_filter {
            Some(ecosystem) => println!(
                "⬆️ Bumping {} ecosystem versions in: {}",
                ecosystem_name(ecosystem),
                repo_path.display()
            ),
            None => println!(
                "⬆️ Bumping all ecosystem versions in: {}",
                repo_path.display()
            ),
        }
    }

    // Load configuration manager to access and update versions
    let mut config_manager = ConfigManager::new()?;

    // Create a clone of the versions config for read operations
    let versions_config_clone = config_manager.versions_config().clone();

    if args.cached {
        if options.verbose {
            println!("📦 Using cached versions from: {}", repo_path.display());
        }
        info!("Using cached versions from: {}", repo_path.display());

        if versions_config_clone.last_checked.is_none() {
            if options.verbose {
                println!("⚠️  No cached versions available. Run without --cached first.");
            }
            info!("No cached versions available. Run without --cached first.");
            return Ok(());
        }

        if options.verbose {
            println!(
                "🕒 Last checked: {}",
                versions_config_clone.last_checked.as_ref().unwrap()
            );
        }
        info!(
            "Last checked: {}",
            versions_config_clone.last_checked.as_ref().unwrap()
        );
    } else {
        info!("Bumping versions in repository at: {}", repo_path.display());
    }

    if options.info {
        let action = match ecosystem_filter {
            Some(ecosystem) => format!(
                "Would bump {} ecosystem versions",
                ecosystem_name(ecosystem)
            ),
            None => "Would bump all ecosystem versions".to_string(),
        };
        println!("ℹ️  {} in repository at {}", action, repo_path.display());
        if options.ai {
            println!(
                "# Version Bumping Analysis\n\nRepository: {}\nAction: {}",
                repo_path.display(),
                action
            );
        }
        return Ok(());
    }

    if options.dry_run {
        let action = match ecosystem_filter {
            Some(ecosystem) => format!(
                "Would bump {} ecosystem versions",
                ecosystem_name(ecosystem)
            ),
            None => "Would bump all ecosystem versions".to_string(),
        };
        println!(
            "🔍 [DRY RUN] {} in repository at {}",
            action,
            repo_path.display()
        );
        info!("Dry run mode: No changes will be made");
        return Ok(());
    }

    // Get the repository
    let repo = git::open_repository(&repo_path).context("Failed to open git repository")?;

    // Process based on whether we're using cached versions or not
    if args.cached {
        // ---- CACHED MODE ----
        // Use cached versions instead of checking online
        let result =
            package_manager::bump_all_versions_from_cache(&repo_path, &versions_config_clone)?;

        if result.is_empty() {
            if options.verbose {
                println!("✅ No package managers found or no updates needed.");
            }
            info!("No package managers found or no updates needed.");
            return Ok(());
        }

        if options.verbose {
            println!("📦 Updated {} package managers", result.len());
        }
        debug!("Updated {} package managers", result.len());

        for pm in &result {
            if options.verbose {
                println!("✅ Updated {} dependencies", pm.name);
            }
            info!("Updated {} dependencies", pm.name);
        }

        // Update GitHub Actions if present
        let github_actions_result = package_manager::update_github_actions_from_cache(
            &repo_path,
            &repo,
            &versions_config_clone,
        )?;

        if github_actions_result.updated {
            if options.verbose {
                println!(
                    "⚙️ Updated {} GitHub Actions workflows",
                    github_actions_result.workflows.len()
                );
            }
            info!(
                "Updated {} GitHub Actions workflows",
                github_actions_result.workflows.len()
            );
        }
    } else {
        // ---- ONLINE MODE ----
        // Check for updates online and update the cache
        let result = package_manager::bump_all_versions(&repo_path)?;

        // Update the versions cache if we found any updates
        if !result.is_empty() {
            let _versions_config = config_manager.versions_config_mut();

            // Record the timestamp
            _versions_config.last_checked = Some(Utc::now().to_rfc3339());

            // Update package manager versions
            for pm in &result {
                let pm_name = pm.name.clone();

                // Create entry if it doesn't exist
                if !_versions_config.package_managers.contains_key(&pm_name) {
                    _versions_config.package_managers.insert(
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

            if options.verbose {
                println!("📦 Updated {} package managers", result.len());
            }
            debug!("Updated {} package managers", result.len());

            for pm in &result {
                if options.verbose {
                    println!("✅ Updated {} dependencies", pm.name);
                }
                info!("Updated {} dependencies", pm.name);
            }
        } else {
            if options.verbose {
                println!("✅ No package managers found or no updates needed.");
            }
            info!("No package managers found or no updates needed.");
            return Ok(());
        }

        // Check for updates online
        let github_actions_result = package_manager::update_github_actions(&repo_path, &repo)?;

        // Update the cache if we found any updates
        if github_actions_result.updated {
            let _versions_config = config_manager.versions_config_mut();

            // TODO: Extract and store actual GitHub Actions versions
            // This would depend on having access to the specific version information

            // Save the updated versions cache again
            config_manager.save_versions()?;

            if options.verbose {
                println!(
                    "⚙️ Updated {} GitHub Actions workflows",
                    github_actions_result.workflows.len()
                );
            }
            info!(
                "Updated {} GitHub Actions workflows",
                github_actions_result.workflows.len()
            );
        }
    }

    // Commit the changes
    git::commit_all(&repo, "Bump dependencies to latest versions")
        .context("Failed to commit version updates")?;

    if options.verbose {
        println!("✅ Version bumping completed successfully!");
    }
    info!("Version bumping completed successfully");

    Ok(())
}

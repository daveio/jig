use crate::cli::UpdateArgs;
use crate::git;
use crate::template;
use crate::utils::paths;
use anyhow::{Context, Result};
use log::{debug, info};

/// Execute the 'update' command
pub fn execute(args: &UpdateArgs, dry_run: bool) -> Result<()> {
    // Determine the repository path
    let repo_path = match &args.repository {
        Some(path) => paths::to_absolute_path(path)?,
        None => paths::get_current_dir()?,
    };

    info!("Updating repository at: {}", repo_path.display());

    if dry_run {
        info!("Dry run mode: No changes will be made");
        info!(
            "Would update template files in repository at {}",
            repo_path.display()
        );
        return Ok(());
    }

    // Get the repository
    let repo = git::open_repository(&repo_path).context("Failed to open git repository")?;

    // Detect language and update template
    let result = template::update_for_repository(&repo_path)?;

    if !result.changed {
        info!("No changes needed. Repository template is up to date.");
        return Ok(());
    }

    debug!("Updated {} files", result.updated_files.len());

    // Commit the changes
    git::commit_all(&repo, "Update template files").context("Failed to commit template updates")?;

    info!("Repository template updated successfully");

    Ok(())
}

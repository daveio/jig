use crate::cli::{CommonOptions, UpdateArgs};
use crate::git;
use crate::template;
use crate::utils::paths;
use anyhow::{Context, Result};
use log::{debug, info};

/// Execute the 'update' command
pub fn execute(args: &UpdateArgs, options: &CommonOptions) -> Result<()> {
    // Determine the repository path
    let repo_path = match &args.repository {
        Some(path) => paths::to_absolute_path(path)?,
        None => paths::get_current_dir()?,
    };

    if options.verbose {
        println!("🔄 Updating repository at: {}", repo_path.display());
    }
    info!("Updating repository at: {}", repo_path.display());

    if options.info {
        println!(
            "ℹ️  Would update template files in repository at {}",
            repo_path.display()
        );
        if options.ai {
            println!("# Repository Update\n\nPath: {}\nAction: Update template files from latest templates", repo_path.display());
        }
        return Ok(());
    }

    if options.dry_run {
        println!(
            "🔍 [DRY RUN] Would update template files in repository at {}",
            repo_path.display()
        );
        info!("Dry run mode: No changes will be made");
        return Ok(());
    }

    // Get the repository
    let repo = git::open_repository(&repo_path).context("Failed to open git repository")?;

    // Detect language and update template
    let result = template::update_for_repository(&repo_path)?;

    if !result.changed {
        if options.verbose {
            println!("✅ No changes needed. Repository template is up to date!");
        }
        info!("No changes needed. Repository template is up to date.");
        return Ok(());
    }

    if options.verbose {
        println!("📝 Updated {} files", result.updated_files.len());
    }
    debug!("Updated {} files", result.updated_files.len());

    // Commit the changes
    git::commit_all(&repo, "Update template files").context("Failed to commit template updates")?;

    if options.verbose {
        println!("✅ Repository template updated successfully!");
    }
    info!("Repository template updated successfully");

    Ok(())
}

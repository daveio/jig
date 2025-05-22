use crate::cli::{CommonOptions, NewArgs};
use crate::git;
use crate::template;
use anyhow::{Context, Result};
use log::{debug, info};
use std::path::Path;

/// Execute the 'new' command
pub fn execute(args: &NewArgs, options: &CommonOptions) -> Result<()> {
    if options.verbose {
        println!(
            "🆕 Creating new repository with language: {}",
            args.language
        );
    }
    info!("Creating new repository with language: {}", args.language);

    if options.info {
        println!(
            "ℹ️  Would create a new repository with {} language",
            args.language
        );
        if options.ai {
            println!("# New Repository Creation\n\nLanguage: {}\nAction: Initialize git repository and create template", args.language);
        }
        return Ok(());
    }

    if options.dry_run {
        println!(
            "🔍 [DRY RUN] Would create a new repository with {} language",
            args.language
        );
        info!("Dry run mode: No changes will be made");
        return Ok(());
    }

    // Initialize git repository
    let repo_path = Path::new(".");
    let repo = git::init_repository(repo_path).context("Failed to initialize git repository")?;

    debug!("Git repository initialized successfully");

    // Create template for the specified language
    template::create_for_language(&args.language, repo_path)
        .context("Failed to create template for language")?;

    // Commit the changes
    git::commit_all(&repo, "Initial commit with template setup")
        .context("Failed to commit template files")?;

    if options.verbose {
        println!(
            "✅ Repository created successfully with {} language!",
            args.language
        );
    }
    info!(
        "Repository created successfully with {} language",
        args.language
    );

    Ok(())
}

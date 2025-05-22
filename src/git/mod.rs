use anyhow::{Context, Result};
use git2::{Repository, Signature, Commit};
use log::{debug, info};
use std::path::Path;

/// Initialize a new Git repository at the given path
pub fn init_repository(path: &Path) -> Result<Repository> {
    debug!("Initializing Git repository at: {}", path.display());

    let repo = Repository::init(path)
        .context("Failed to initialize Git repository")?;

    debug!("Git repository initialized successfully");

    Ok(repo)
}

/// Open an existing Git repository at the given path
pub fn open_repository(path: &Path) -> Result<Repository> {
    debug!("Opening Git repository at: {}", path.display());

    let repo = Repository::open(path)
        .context("Failed to open Git repository")?;

    debug!("Git repository opened successfully");

    Ok(repo)
}

/// Commit all changes in the repository
pub fn commit_all(repo: &Repository, message: &str) -> Result<()> {
    debug!("Committing all changes with message: {}", message);

    // Get the index
    let mut index = repo.index()
        .context("Failed to get repository index")?;

    // Add all changes
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)
        .context("Failed to add all changes to index")?;

    index.write()
        .context("Failed to write index")?;

    let oid = index.write_tree()
        .context("Failed to write tree")?;

    let tree = repo.find_tree(oid)
        .context("Failed to find tree")?;

    // Get the author/committer information
    let signature = get_signature(repo)?;

    // Get parent commit if it exists
    let parent_commits = match repo.head() {
        Ok(head) => {
            let object = head.resolve()?.peel(git2::ObjectType::Commit)?;
            if let Ok(commit) = object.into_commit() {
                vec![commit]
            } else {
                vec![]
            }
        },
        Err(_) => vec![],
    };

    // Convert commits to references for the parent slice
    let parent_refs: Vec<&Commit> = parent_commits.iter().collect();

    // Create the commit
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &parent_refs,
    )
    .context("Failed to create commit")?;

    info!("Changes committed successfully");

    Ok(())
}

/// Get the signature for commits
fn get_signature(repo: &Repository) -> Result<Signature<'static>> {
    // Try to get the signature from the repository config
    let config = repo.config()?;

    let name = match config.get_string("user.name") {
        Ok(name) => name,
        Err(_) => {
            // Fallback to environment variables or default
            match std::env::var("GIT_AUTHOR_NAME") {
                Ok(name) => name,
                Err(_) => "Jig Tool".to_string(),
            }
        }
    };

    let email = match config.get_string("user.email") {
        Ok(email) => email,
        Err(_) => {
            // Fallback to environment variables or default
            match std::env::var("GIT_AUTHOR_EMAIL") {
                Ok(email) => email,
                Err(_) => "jig@example.com".to_string(),
            }
        }
    };

    let signature = Signature::now(&name, &email)
        .context("Failed to create signature")?;

    Ok(signature)
}

/// Get the default branch name (main or master) for the repository
pub fn get_default_branch(repo: &Repository) -> Result<String> {
    let config = repo.config()?;

    // Try to get the init.defaultBranch config
    match config.get_string("init.defaultBranch") {
        Ok(name) => return Ok(name),
        Err(_) => {
            // Check if main or master exists
            for branch_name in &["main", "master"] {
                if repo.find_branch(branch_name, git2::BranchType::Local).is_ok() {
                    return Ok(branch_name.to_string());
                }
            }

            // Fallback to main
            Ok("main".to_string())
        }
    }
}

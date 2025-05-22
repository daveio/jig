use anyhow::{Context, Result};
use git2::{Commit, Repository, Signature};
use log::{debug, info};
use std::path::Path;

/// Initialize a new Git repository at the given path
pub fn init_repository(path: &Path) -> Result<Repository> {
    debug!("Initializing Git repository at: {}", path.display());

    let repo = Repository::init(path).context("Failed to initialize Git repository")?;

    debug!("Git repository initialized successfully");

    Ok(repo)
}

/// Open an existing Git repository at the given path
pub fn open_repository(path: &Path) -> Result<Repository> {
    debug!("Opening Git repository at: {}", path.display());

    let repo = Repository::open(path).context("Failed to open Git repository")?;

    debug!("Git repository opened successfully");

    Ok(repo)
}

/// Commit all changes in the repository
pub fn commit_all(repo: &Repository, message: &str) -> Result<()> {
    debug!("Committing all changes with message: {}", message);

    // Get the index
    let mut index = repo.index().context("Failed to get repository index")?;

    // Add all changes
    index
        .add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)
        .context("Failed to add all changes to index")?;

    index.write().context("Failed to write index")?;

    let oid = index.write_tree().context("Failed to write tree")?;

    let tree = repo.find_tree(oid).context("Failed to find tree")?;

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
        }
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
    // Try environment variables first (for testing), then fall back to git config
    let name = match std::env::var("GIT_AUTHOR_NAME") {
        Ok(name) => name,
        Err(_) => {
            // Fallback to repository config
            let config = repo.config()?;
            match config.get_string("user.name") {
                Ok(name) => name,
                Err(_) => "Jig Tool".to_string(),
            }
        }
    };

    let email = match std::env::var("GIT_AUTHOR_EMAIL") {
        Ok(email) => email,
        Err(_) => {
            // Fallback to repository config
            let config = repo.config()?;
            match config.get_string("user.email") {
                Ok(email) => email,
                Err(_) => "jig@example.com".to_string(),
            }
        }
    };

    let signature = Signature::now(&name, &email).context("Failed to create signature")?;

    Ok(signature)
}

/// Get the default branch name (main or master) for the repository
pub fn get_default_branch(repo: &Repository) -> Result<String> {
    let config = repo.config()?;

    // Try to get the init.defaultBranch config
    match config.get_string("init.defaultBranch") {
        Ok(name) => Ok(name),
        Err(_) => {
            // Check if main or master exists
            for branch_name in &["main", "master"] {
                if repo
                    .find_branch(branch_name, git2::BranchType::Local)
                    .is_ok()
                {
                    return Ok(branch_name.to_string());
                }
            }

            // Fallback to main
            Ok("main".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};
    use tempfile::tempdir;

    #[test]
    fn test_init_repository() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();

        let result = init_repository(repo_path);
        assert!(result.is_ok());

        let _repo = result.unwrap();

        // Verify that .git directory exists
        let git_dir = repo_path.join(".git");
        assert!(git_dir.exists());
        assert!(git_dir.is_dir());
    }

    #[test]
    fn test_open_repository() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();

        // First initialize a repository
        let _repo = init_repository(repo_path).unwrap();

        // Then try to open it
        let result = open_repository(repo_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_open_repository_nonexistent() {
        let temp_dir = tempdir().unwrap();
        let non_repo_path = temp_dir.path().join("not_a_repo");

        let result = open_repository(&non_repo_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_commit_all() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();

        // Initialize repository and set up git config
        let repo = init_repository(repo_path).unwrap();

        // Create a test file
        let test_file = repo_path.join("test.txt");
        fs::write(&test_file, "Hello, world!").unwrap();

        // Test commit
        let result = commit_all(&repo, "Initial commit");
        assert!(result.is_ok());

        // Verify commit was created
        assert!(repo.head().is_ok());
    }

    #[test]
    fn test_get_signature_with_defaults() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();
        let repo = init_repository(repo_path).unwrap();

        // Clear any environment variables that might interfere
        unsafe {
            env::remove_var("GIT_AUTHOR_NAME");
            env::remove_var("GIT_AUTHOR_EMAIL");
        }

        let result = get_signature(&repo);
        assert!(result.is_ok());

        let signature = result.unwrap();
        // Should use defaults if no config is set
        assert!(!signature.name().unwrap_or("").is_empty());
        assert!(!signature.email().unwrap_or("").is_empty());
    }

    #[test]
    fn test_get_signature_with_env_vars() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();
        let repo = init_repository(repo_path).unwrap();

        // Clear repo config first to make sure env vars take precedence
        {
            let mut config = repo.config().unwrap();
            let _ = config.remove("user.name");
            let _ = config.remove("user.email");
        }

        // Set environment variables
        unsafe {
            env::set_var("GIT_AUTHOR_NAME", "Test Author");
            env::set_var("GIT_AUTHOR_EMAIL", "test@example.com");
        }

        let result = get_signature(&repo);
        assert!(result.is_ok());

        let signature = result.unwrap();
        assert_eq!(signature.name().unwrap(), "Test Author");
        assert_eq!(signature.email().unwrap(), "test@example.com");

        // Clean up
        unsafe {
            env::remove_var("GIT_AUTHOR_NAME");
            env::remove_var("GIT_AUTHOR_EMAIL");
        }
    }

    #[test]
    fn test_get_default_branch() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();
        let repo = init_repository(repo_path).unwrap();

        let result = get_default_branch(&repo);
        assert!(result.is_ok());

        let branch_name = result.unwrap();
        // Should default to "main" or "master"
        assert!(branch_name == "main" || branch_name == "master");
    }

    #[test]
    fn test_roundtrip_repository() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();

        // Initialize repository
        let repo = init_repository(repo_path).unwrap();

        // Create and commit a file
        let test_file = repo_path.join("README.md");
        fs::write(&test_file, "# Test Repository").unwrap();

        commit_all(&repo, "Add README").unwrap();

        // Close and reopen repository
        drop(repo);
        let reopened = open_repository(repo_path).unwrap();

        // Verify we can access the commit
        assert!(reopened.head().is_ok());
        let default_branch = get_default_branch(&reopened).unwrap();
        assert!(!default_branch.is_empty());
    }
}

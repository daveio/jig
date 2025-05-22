use anyhow::{Context, Result};
use std::env;
use std::path::{Path, PathBuf};

/// Get the current working directory as a PathBuf
pub fn get_current_dir() -> Result<PathBuf> {
    env::current_dir().context("Failed to get current directory")
}

/// Convert a potentially relative path to an absolute path
pub fn to_absolute_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let current_dir = get_current_dir()?;
        Ok(current_dir.join(path))
    }
}

/// Normalize a path to ensure it's in a consistent format
pub fn normalize_path(path: &Path) -> PathBuf {
    dunce::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

/// Check if a path exists
pub fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// Create directories recursively
pub fn create_directories(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path).context("Failed to create directories")
}

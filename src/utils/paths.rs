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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_get_current_dir() {
        let result = get_current_dir();
        assert!(result.is_ok());
        let current_dir = result.unwrap();
        assert!(current_dir.is_absolute());
        assert!(current_dir.exists());
    }

    #[test]
    fn test_to_absolute_path_with_absolute_path() {
        let temp_dir = tempdir().unwrap();
        let absolute_path = temp_dir.path();

        let result = to_absolute_path(absolute_path).unwrap();
        assert_eq!(result, absolute_path);
        assert!(result.is_absolute());
    }

    #[test]
    fn test_to_absolute_path_with_relative_path() {
        let relative_path = Path::new("relative/path");
        let result = to_absolute_path(relative_path).unwrap();

        assert!(result.is_absolute());
        assert!(result.to_string_lossy().contains("relative/path"));
    }

    #[test]
    fn test_normalize_path_existing() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        let normalized = normalize_path(temp_path);
        assert!(normalized.is_absolute());
        // The normalized path should still exist
        assert!(normalized.exists());
    }

    #[test]
    fn test_normalize_path_nonexistent() {
        let nonexistent = Path::new("/this/path/does/not/exist");
        let normalized = normalize_path(nonexistent);
        // Should return the original path if canonicalization fails
        assert_eq!(normalized, nonexistent);
    }

    #[test]
    fn test_path_exists_true() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        assert!(path_exists(temp_path));
    }

    #[test]
    fn test_path_exists_false() {
        let nonexistent = Path::new("/this/path/definitely/does/not/exist");
        assert!(!path_exists(nonexistent));
    }

    #[test]
    fn test_create_directories() {
        let temp_dir = tempdir().unwrap();
        let nested_path = temp_dir.path().join("deeply").join("nested").join("path");

        let result = create_directories(&nested_path);
        assert!(result.is_ok());
        assert!(nested_path.exists());
        assert!(nested_path.is_dir());
    }

    #[test]
    fn test_create_directories_existing() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        // Creating an existing directory should succeed
        let result = create_directories(temp_path);
        assert!(result.is_ok());
    }
}

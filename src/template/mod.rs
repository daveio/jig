use crate::utils::paths;
use anyhow::{Context, Result};
use log::{debug, info};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tera::Tera;
use walkdir::WalkDir;

pub mod component;
pub mod language;
#[cfg(test)]
mod tests;

/// Result of template updates
pub struct UpdateResult {
    pub changed: bool,
    pub updated_files: Vec<PathBuf>,
}

/// Create a template for the specified language
pub fn create_for_language(language: &str, repo_path: &Path) -> Result<()> {
    info!("Creating template for language: {}", language);

    // Get the language-specific template
    let template_dir = get_template_dir_for_language(language)?;

    // Process and copy the template to the repository
    process_template_dir(&template_dir, repo_path, language)?;

    info!("Template created successfully for {} language", language);

    Ok(())
}

/// Update the template for an existing repository
pub fn update_for_repository(repo_path: &Path) -> Result<UpdateResult> {
    info!(
        "Updating template for repository at: {}",
        repo_path.display()
    );

    // Detect the language of the repository
    let detected_language = language::detect_language(repo_path)?;
    debug!("Detected language: {}", detected_language);

    // Get the template directory for the detected language
    let template_dir = get_template_dir_for_language(&detected_language)?;

    // Check which files need updating
    let update_result =
        check_and_update_template_files(&template_dir, repo_path, &detected_language)?;

    Ok(update_result)
}

/// Get the template directory for a specific language
fn get_template_dir_for_language(language: &str) -> Result<PathBuf> {
    // Get the templates directory from the executable's location
    let exe_dir = std::env::current_exe()?
        .parent()
        .context("Failed to get executable directory")?
        .to_path_buf();

    let templates_dir = exe_dir.join("templates");

    // Check if we have the template directory locally (development mode)
    let local_templates_dir = paths::get_current_dir()?.join("templates");

    let templates_path = if local_templates_dir.exists() && local_templates_dir.is_dir() {
        local_templates_dir
    } else if templates_dir.exists() && templates_dir.is_dir() {
        templates_dir
    } else {
        anyhow::bail!("Templates directory not found");
    };

    let language_template_dir = templates_path.join(language.to_lowercase());

    if !language_template_dir.exists() || !language_template_dir.is_dir() {
        anyhow::bail!("Template for language '{}' not found", language);
    }

    Ok(language_template_dir)
}

/// Get the templates root directory
fn get_templates_root_dir() -> Result<PathBuf> {
    // Get the templates directory from the executable's location
    let exe_dir = std::env::current_exe()?
        .parent()
        .context("Failed to get executable directory")?
        .to_path_buf();

    let templates_dir = exe_dir.join("templates");

    // Check if we have the template directory locally (development mode)
    let local_templates_dir = paths::get_current_dir()?.join("templates");

    let templates_path = if local_templates_dir.exists() && local_templates_dir.is_dir() {
        local_templates_dir
    } else if templates_dir.exists() && templates_dir.is_dir() {
        templates_dir
    } else {
        anyhow::bail!("Templates directory not found");
    };

    Ok(templates_path)
}

/// Process and copy template files to the repository
fn process_template_dir(template_dir: &Path, repo_path: &Path, language: &str) -> Result<()> {
    debug!("Processing template directory: {}", template_dir.display());

    // Get the templates root directory
    let templates_root = get_templates_root_dir()?;

    // Create a Tera instance for template rendering
    let mut tera = Tera::default();

    // Variables for template rendering
    let mut context = tera::Context::new();
    context.insert("language", language);

    // Get the project name from the repository path
    let project_name = repo_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project")
        .to_string();

    context.insert("project_name", &project_name);

    // Walk through the template directory
    for entry in WalkDir::new(template_dir) {
        let entry = entry?;
        let entry_path = entry.path();

        // Skip directories
        if entry_path.is_dir() {
            continue;
        }

        // Get the relative path from the template directory
        let rel_path = entry_path.strip_prefix(template_dir)?;
        let target_path = repo_path.join(rel_path);

        // Create parent directories if needed
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Check if this is a placeholder file
        if component::is_placeholder_file(entry_path) {
            // Process the placeholder file
            component::process_placeholder_file(
                entry_path,
                &target_path,
                &templates_root,
                &context,
            )?;
        } else if entry_path.extension().is_some_and(|ext| ext == "tera") {
            // Process a regular template file
            // Read the template content
            let template_content = fs::read_to_string(entry_path)?;

            // Process the template
            let processed_content = tera.render_str(&template_content, &context)?;

            // Write the processed content to the target path without the .tera extension
            let target_path_without_tera = target_path.with_file_name(
                target_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("")
                    .replace(".tera", ""),
            );

            fs::write(target_path_without_tera, processed_content)?;
        } else {
            // Just copy the file as-is
            fs::copy(entry_path, target_path)?;
        }
    }

    debug!("Template directory processed successfully");

    Ok(())
}

/// Check which template files need updating and update them
fn check_and_update_template_files(
    template_dir: &Path,
    repo_path: &Path,
    language: &str,
) -> Result<UpdateResult> {
    debug!("Checking for template updates");

    // Get the templates root directory
    let templates_root = get_templates_root_dir()?;

    let mut updated_files = Vec::new();
    let mut existing_files = HashSet::new();

    // Create a Tera instance for template rendering
    let mut tera = Tera::default();

    // Variables for template rendering
    let mut context = tera::Context::new();
    context.insert("language", language);

    // Get the project name from the repository path
    let project_name = repo_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project")
        .to_string();

    context.insert("project_name", &project_name);

    // Collect existing files in the repository
    for entry in WalkDir::new(repo_path) {
        let entry = entry?;
        if entry.path().is_file() {
            let rel_path = entry.path().strip_prefix(repo_path)?;
            existing_files.insert(rel_path.to_path_buf());
        }
    }

    // Walk through the template directory
    for entry in WalkDir::new(template_dir) {
        let entry = entry?;
        let entry_path = entry.path();

        // Skip directories
        if entry_path.is_dir() {
            continue;
        }

        // Get the relative path from the template directory
        let rel_path = entry_path.strip_prefix(template_dir)?;
        let target_path = repo_path.join(rel_path);

        // Check if the file is a placeholder file
        let is_placeholder = component::is_placeholder_file(entry_path);

        // Check if the file should be processed as a template
        let is_template =
            !is_placeholder && entry_path.extension().is_some_and(|ext| ext == "tera");

        let target_path_without_ext = if is_template {
            target_path.with_file_name(
                target_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("")
                    .replace(".tera", ""),
            )
        } else {
            target_path.clone()
        };

        // Handle special case for placeholder files
        let rel_path_to_check = if is_placeholder {
            if rel_path.to_str() == Some("gitignore") {
                Path::new(".gitignore").to_path_buf()
            } else if rel_path.to_str() == Some("workflows") {
                // Workflows are in .github/workflows directory
                // We'll handle this as a special case by not checking for existing files
                Path::new(".github/workflows").join(format!("{}.yml", language.to_lowercase()))
            } else {
                rel_path.to_path_buf()
            }
        } else if is_template {
            Path::new(rel_path).with_file_name(
                rel_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("")
                    .replace(".tera", ""),
            )
        } else {
            rel_path.to_path_buf()
        };

        if !existing_files.contains(&rel_path_to_check) || is_placeholder {
            // File doesn't exist or is a placeholder (we always update placeholders)
            debug!("Creating/updating file: {}", rel_path_to_check.display());

            // Create parent directories if needed
            if let Some(parent) = target_path_without_ext.parent() {
                fs::create_dir_all(parent)?;
            }

            if is_placeholder {
                // Process the placeholder file
                component::process_placeholder_file(
                    entry_path,
                    &target_path,
                    &templates_root,
                    &context,
                )?;

                // Add the generated files to updated_files
                if rel_path.to_str() == Some("gitignore") {
                    updated_files.push(repo_path.join(".gitignore"));
                } else if rel_path.to_str() == Some("workflows") {
                    updated_files.push(
                        repo_path
                            .join(".github/workflows")
                            .join(format!("{}.yml", language.to_lowercase())),
                    );
                }
            } else if is_template {
                // Process and write the template
                let template_content = fs::read_to_string(entry_path)?;
                let processed_content = tera.render_str(&template_content, &context)?;
                fs::write(&target_path_without_ext, processed_content)?;
                updated_files.push(target_path_without_ext);
            } else {
                // Just copy the file as-is
                fs::copy(entry_path, &target_path_without_ext)?;
                updated_files.push(target_path_without_ext);
            }
        }
    }

    let changed = !updated_files.is_empty();

    Ok(UpdateResult {
        changed,
        updated_files,
    })
}

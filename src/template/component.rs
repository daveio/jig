use anyhow::{Context, Result};
use log::{debug, error};
use std::fs;
use std::path::{Path, PathBuf};
use tera::Tera;
use yaml_rust2::{Yaml, YamlLoader};

/// Get the shared components directory
pub fn get_shared_components_dir(templates_path: &Path) -> PathBuf {
    templates_path.join("shared")
}

/// Check if a file is a placeholder file (no extension, not a directory)
pub fn is_placeholder_file(path: &Path) -> bool {
    path.is_file() && path.extension().is_none()
}

/// Process a placeholder file
pub fn process_placeholder_file(
    placeholder_path: &Path,
    target_path: &Path,
    templates_path: &Path,
    context: &tera::Context,
) -> Result<()> {
    let filename = placeholder_path
        .file_name()
        .and_then(|name| name.to_str())
        .context("Failed to get placeholder filename")?;

    debug!(
        "Processing placeholder file: {}",
        placeholder_path.display()
    );

    match filename {
        "gitignore" => {
            process_gitignore_placeholder(placeholder_path, target_path, templates_path)?
        }
        "workflows" => {
            process_workflows_placeholder(placeholder_path, target_path, templates_path, context)?
        }
        _ => {
            // Unknown placeholder type, just copy it
            debug!("Unknown placeholder type: {}, copying as-is", filename);
            fs::copy(placeholder_path, target_path)?;
        }
    }

    Ok(())
}

/// Process a gitignore placeholder file
fn process_gitignore_placeholder(
    placeholder_path: &Path,
    target_path: &Path,
    templates_path: &Path,
) -> Result<()> {
    debug!(
        "Processing gitignore placeholder: {}",
        placeholder_path.display()
    );

    // Read the placeholder file to get the list of components
    let placeholder_content = fs::read_to_string(placeholder_path)?;
    let shared_dir = get_shared_components_dir(templates_path).join("gitignore");

    // Split the content by lines and process each component
    let mut combined_content = String::new();

    for component in placeholder_content.lines() {
        let component = component.trim();
        if component.is_empty() {
            continue;
        }

        let component_path = shared_dir.join(format!("{}.gitignore", component));
        debug!(
            "Including gitignore component: {}",
            component_path.display()
        );

        if component_path.exists() && component_path.is_file() {
            let component_content = fs::read_to_string(&component_path)?;
            combined_content.push_str(&component_content);
            combined_content.push('\n');
        } else {
            error!(
                "Gitignore component not found: {}",
                component_path.display()
            );
        }
    }

    // Create the target .gitignore file
    let target_gitignore = target_path.with_file_name(".gitignore");
    debug!(
        "Writing combined gitignore to: {}",
        target_gitignore.display()
    );
    fs::write(target_gitignore, combined_content)?;

    Ok(())
}

/// Process a workflows placeholder file
fn process_workflows_placeholder(
    placeholder_path: &Path,
    target_path: &Path,
    templates_path: &Path,
    parent_context: &tera::Context,
) -> Result<()> {
    debug!(
        "Processing workflows placeholder: {}",
        placeholder_path.display()
    );

    // Read the placeholder file to get the workflow configuration
    let placeholder_content = fs::read_to_string(placeholder_path)?;
    let yaml_docs = YamlLoader::load_from_str(&placeholder_content)?;

    if yaml_docs.is_empty() {
        return Ok(());
    }

    let yaml = &yaml_docs[0];
    let shared_dir = get_shared_components_dir(templates_path)
        .join("github")
        .join("workflows");

    // Create the target directory
    let target_dir = target_path
        .parent()
        .unwrap()
        .join(".github")
        .join("workflows");
    fs::create_dir_all(&target_dir)?;

    // Process each workflow configuration
    if let Some(base) = yaml["base"].as_hash() {
        let workflow_name = base
            .get(&Yaml::from_str("workflow_name"))
            .and_then(|name| name.as_str())
            .unwrap_or("Workflow");

        let jobs_template = base
            .get(&Yaml::from_str("jobs_template"))
            .and_then(|template| template.as_str())
            .unwrap_or("jobs");

        let env = base
            .get(&Yaml::from_str("env"))
            .and_then(|env| env.as_str())
            .unwrap_or("");

        // Read the base template
        let base_template_path = shared_dir.join("base.yml.tera");
        let base_template = fs::read_to_string(&base_template_path)?;

        // Read the jobs template
        let jobs_template_path = shared_dir.join(format!("{}.yml.tera", jobs_template));
        let jobs_template = fs::read_to_string(&jobs_template_path)?;

        // Create a new context with workflow-specific variables
        let mut context = parent_context.clone();
        context.insert("workflow_name", workflow_name);
        context.insert("jobs", &jobs_template);
        context.insert("env", env);

        // Render the template
        let mut tera = Tera::default();
        let rendered = tera.render_str(&base_template, &context)?;

        // Write the rendered template to the target file
        let target_file = target_dir.join(format!("{}.yml", workflow_name.to_lowercase()));
        debug!("Writing workflow to: {}", target_file.display());
        fs::write(target_file, rendered)?;
    }

    Ok(())
}

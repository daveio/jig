#[cfg(test)]
mod tests {
    use crate::template::component;
    use std::fs;
    use tempfile::tempdir;
    use tera::Context;

    #[test]
    fn test_is_placeholder_file() {
        // Create a temporary directory for testing
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        // Create placeholder files
        let gitignore_path = temp_path.join("gitignore");
        fs::write(&gitignore_path, "common\nrust").unwrap();

        // Create non-placeholder files
        let tera_path = temp_path.join("file.tera");
        fs::write(&tera_path, "content").unwrap();

        let regular_path = temp_path.join("file.txt");
        fs::write(&regular_path, "content").unwrap();

        // Test the function
        assert!(component::is_placeholder_file(&gitignore_path));
        assert!(!component::is_placeholder_file(&tera_path));
        assert!(!component::is_placeholder_file(&regular_path));
    }

    #[test]
    fn test_process_gitignore_placeholder() {
        // Create a temporary directory structure for testing
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        // Create templates directory structure
        let templates_dir = temp_path.join("templates");
        let shared_dir = templates_dir.join("shared");
        let gitignore_dir = shared_dir.join("gitignore");
        let rust_dir = templates_dir.join("rust");

        fs::create_dir_all(&gitignore_dir).unwrap();
        fs::create_dir_all(&rust_dir).unwrap();

        // Create shared gitignore components
        fs::write(
            gitignore_dir.join("common.gitignore"),
            "# Common gitignore patterns\n.DS_Store\n",
        )
        .unwrap();

        fs::write(
            gitignore_dir.join("rust.gitignore"),
            "# Rust gitignore patterns\n/target/\nCargo.lock\n",
        )
        .unwrap();

        // Create placeholder file
        let placeholder_path = rust_dir.join("gitignore");
        fs::write(&placeholder_path, "common\nrust").unwrap();

        // Create target directory
        let target_dir = temp_path.join("target");
        fs::create_dir_all(&target_dir).unwrap();

        // Process the placeholder
        let target_path = target_dir.join("gitignore");
        let context = Context::new();

        component::process_placeholder_file(
            &placeholder_path,
            &target_path,
            &templates_dir,
            &context,
        )
        .unwrap();

        // Check that the target file was created correctly
        let target_gitignore = target_dir.join(".gitignore");
        assert!(target_gitignore.exists());

        let content = fs::read_to_string(&target_gitignore).unwrap();
        assert!(content.contains("# Common gitignore patterns"));
        assert!(content.contains(".DS_Store"));
        assert!(content.contains("# Rust gitignore patterns"));
        assert!(content.contains("/target/"));
        assert!(content.contains("Cargo.lock"));
    }

    #[test]
    fn test_process_workflows_placeholder() {
        // Create a temporary directory structure for testing
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        // Create templates directory structure
        let templates_dir = temp_path.join("templates");
        let shared_dir = templates_dir.join("shared");
        let github_dir = shared_dir.join("github");
        let workflows_dir = github_dir.join("workflows");
        let rust_dir = templates_dir.join("rust");

        fs::create_dir_all(&workflows_dir).unwrap();
        fs::create_dir_all(&rust_dir).unwrap();

        // Create shared workflow components
        fs::write(
            workflows_dir.join("base.yml.tera"),
            "name: {{ workflow_name }}\n\non:\n  push:\n    branches: [ \"main\" ]\n\njobs:\n{{ jobs | indent(width=2) }}",
        )
        .unwrap();

        fs::write(
            workflows_dir.join("rust_jobs.yml.tera"),
            "build:\n  runs-on: ubuntu-latest\n  steps:\n  - uses: actions/checkout@v4",
        )
        .unwrap();

        // Create placeholder file
        let placeholder_path = rust_dir.join("workflows");
        fs::write(
            &placeholder_path,
            "base:\n  workflow_name: Rust\n  jobs_template: rust_jobs",
        )
        .unwrap();

        // Create target directory
        let target_dir = temp_path.join("target");
        fs::create_dir_all(&target_dir).unwrap();

        // Create context
        let mut context = Context::new();
        context.insert("language", "rust");
        context.insert("project_name", "test-project");

        // Process the placeholder
        let target_path = target_dir.join("workflows");
        component::process_placeholder_file(
            &placeholder_path,
            &target_path,
            &templates_dir,
            &context,
        )
        .unwrap();

        // Check that the target file was created correctly
        let target_workflow = target_dir
            .join(".github")
            .join("workflows")
            .join("rust.yml");
        assert!(target_workflow.exists());

        let content = fs::read_to_string(&target_workflow).unwrap();
        assert!(content.contains("name: Rust"));
        assert!(content.contains("branches: [ \"main\" ]"));
        assert!(content.contains("build:"));
        assert!(content.contains("runs-on: ubuntu-latest"));
        assert!(content.contains("uses: actions/checkout@v4"));
    }
}
